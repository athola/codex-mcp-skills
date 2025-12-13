//! Diagnostics for Codex MCP setup.
//!
//! Inspects and validates MCP server configurations (JSON and TOML)
//! to diagnose common setup issues.

use anyhow::Result;
use skrills_state::home_dir;
use std::fs;
use std::path::Path;

/// Validates an MCP server entry and prints diagnostics.
fn validate_mcp_entry(
    entry: &serde_json::Value,
    expected_cmd: &Path,
    config_path: &Path,
    file_label: &str,
) {
    let typ = entry
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("<missing>");
    let cmd = entry
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("<missing>");

    let args_display = entry
        .get("args")
        .map(|v| format!("{:?}", v))
        .unwrap_or_else(|| "None".to_string());
    println!(
        "{file_label}: type={typ} command={cmd} args={args_display} ({})",
        config_path.display()
    );

    if typ != "stdio" {
        println!("  ! expected type=\"stdio\"");
    }
    if file_label.contains("json") && Path::new(cmd) != expected_cmd {
        println!("  i command differs; ensure binary path is correct and executable");
    }
    if !Path::new(cmd).exists() {
        println!("  ! command path does not exist on disk");
    }
}

/// Inspects the MCP servers JSON configuration file.
fn inspect_mcp_json(mcp_path: &Path, expected_cmd: &Path) -> Result<()> {
    if !mcp_path.exists() {
        println!("mcp_servers.json: not found at {}", mcp_path.display());
        return Ok(());
    }

    let raw = fs::read_to_string(mcp_path)?;
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(json) => {
            if let Some(entry) = json.get("mcpServers").and_then(|m| m.get("skrills")) {
                validate_mcp_entry(entry, expected_cmd, mcp_path, "mcp_servers.json");
            } else {
                println!(
                    "mcp_servers.json: missing skrills entry ({})",
                    mcp_path.display()
                );
            }
        }
        Err(e) => println!(
            "mcp_servers.json: failed to parse ({:?}): {}",
            mcp_path.display(),
            e
        ),
    }
    Ok(())
}

/// Inspects the Codex config TOML file.
fn inspect_config_toml(cfg_path: &Path, expected_cmd: &Path) -> Result<()> {
    if !cfg_path.exists() {
        println!("config.toml:    not found at {}", cfg_path.display());
        return Ok(());
    }

    let raw = fs::read_to_string(cfg_path)?;
    match toml::from_str::<toml::Value>(&raw) {
        Ok(toml_val) => {
            let entry = toml_val.get("mcp_servers").and_then(|m| m.get("skrills"));
            if let Some(e) = entry {
                let json_entry = serde_json::to_value(e).unwrap_or(serde_json::Value::Null);
                validate_mcp_entry(&json_entry, expected_cmd, cfg_path, "config.toml   ");
            } else {
                println!(
                    "config.toml:    missing [mcp_servers.skrills] ({})",
                    cfg_path.display()
                );
            }
        }
        Err(e) => println!(
            "config.toml:    failed to parse ({:?}): {}",
            cfg_path.display(),
            e
        ),
    }
    Ok(())
}

/// Runs diagnostics on Codex MCP configuration files.
///
/// Inspects `~/.codex/mcp_servers.json` and `~/.codex/config.toml` to validate `skrills` server configuration and identify common issues.
pub fn doctor_report() -> Result<()> {
    let home = home_dir()?;
    let mcp_path = home.join(".codex/mcp_servers.json");
    let cfg_path = home.join(".codex/config.toml");
    let expected_cmd = home.join(".codex/bin/skrills");

    println!("== skrills doctor ==");

    inspect_mcp_json(&mcp_path, &expected_cmd)?;
    inspect_config_toml(&cfg_path, &expected_cmd)?;

    println!("Hint: Codex CLI raises 'missing field `type`' when either file lacks type=\"stdio\" for skrills.");
    Ok(())
}
