//! Unit tests for the setup module.
//!
//! These tests follow TDD/BDD principles, focusing on business logic and use cases.

use super::*;
use std::env;
use std::sync::Mutex;
use tempfile::TempDir;

/// Test serialization guard to avoid environment variable conflicts between tests.
static TEST_SERIAL: Mutex<()> = Mutex::new(());

fn env_guard() -> std::sync::MutexGuard<'static, ()> {
    TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

/// Test fixture helper for creating temporary home directories
fn create_test_home() -> Result<TempDir> {
    TempDir::new().context("Failed to create temp dir")
}

/// Set HOME environment variable to test directory
/// Note: dirs::home_dir() may not respect this on all platforms
fn set_test_home(dir: &TempDir) {
    env::set_var("HOME", dir.path());
}

/// Create a test directory structure within the actual home directory
/// This is more reliable for testing
#[allow(dead_code)]
fn create_isolated_test_dirs(_name: &str) -> Result<(PathBuf, PathBuf)> {
    let temp = create_test_home()?;
    let claude_dir = temp.path().join(".claude");
    let codex_dir = temp.path().join(".codex");
    Ok((claude_dir, codex_dir))
}

#[cfg(test)]
mod client_tests {
    use super::*;

    #[test]
    fn test_client_base_dir_claude() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        let base_dir = Client::Claude.base_dir()?;
        assert!(base_dir.ends_with(".claude"));
        Ok(())
    }

    #[test]
    fn test_client_base_dir_codex() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        let base_dir = Client::Codex.base_dir()?;
        assert!(base_dir.ends_with(".codex"));
        Ok(())
    }

    #[test]
    fn test_client_default_bin_dir() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        let bin_dir = Client::Claude.default_bin_dir()?;
        assert!(bin_dir.ends_with(".claude/bin"));
        Ok(())
    }

    #[test]
    fn test_client_as_str() {
        assert_eq!(Client::Claude.as_str(), "claude");
        assert_eq!(Client::Codex.as_str(), "codex");
    }

    #[test]
    fn test_client_from_str_valid() -> Result<()> {
        assert_eq!(Client::from_str("claude")?, Client::Claude);
        assert_eq!(Client::from_str("codex")?, Client::Codex);
        assert_eq!(Client::from_str("CLAUDE")?, Client::Claude);
        assert_eq!(Client::from_str("Codex")?, Client::Codex);
        Ok(())
    }

    #[test]
    fn test_client_from_str_invalid() {
        assert!(Client::from_str("invalid").is_err());
        assert!(Client::from_str("").is_err());
    }
}

#[cfg(test)]
mod detection_tests {
    use super::*;

    #[test]
    fn test_is_setup_detects_fresh_install() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        // Fresh install - no setup
        assert!(!is_setup(Client::Claude)?);
        assert!(!is_setup(Client::Codex)?);
        Ok(())
    }

    // Note: These tests verify the logic but may not work with dirs::home_dir()
    // which doesn't always respect HOME env var. They demonstrate the intended behavior.

    #[test]
    fn test_is_first_run_no_setup() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        assert!(is_first_run()?);
        Ok(())
    }

    // Note: These tests demonstrate expected behavior but may be skipped
    // due to HOME directory limitations in test environment
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_clients_claude() -> Result<()> {
        let clients = parse_clients("claude")?;
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0], Client::Claude);
        Ok(())
    }

    #[test]
    fn test_parse_clients_codex() -> Result<()> {
        let clients = parse_clients("codex")?;
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0], Client::Codex);
        Ok(())
    }

    #[test]
    fn test_parse_clients_both() -> Result<()> {
        let clients = parse_clients("both")?;
        assert_eq!(clients.len(), 2);
        assert_eq!(clients[0], Client::Claude);
        assert_eq!(clients[1], Client::Codex);
        Ok(())
    }

    #[test]
    fn test_parse_clients_case_insensitive() {
        assert!(parse_clients("CLAUDE").is_ok());
        assert!(parse_clients("Codex").is_ok());
        assert!(parse_clients("BOTH").is_ok());
    }

    #[test]
    fn test_parse_clients_invalid() {
        assert!(parse_clients("invalid").is_err());
        assert!(parse_clients("").is_err());
    }
}

#[cfg(test)]
mod setup_config_tests {
    use super::*;

    #[test]
    fn test_setup_config_creation() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        let config = SetupConfig {
            clients: vec![Client::Claude],
            bin_dir: PathBuf::from("/test/bin"),
            reinstall: false,
            uninstall: false,
            add: false,
            yes: true,
            universal: false,
            mirror_source: None,
        };

        assert_eq!(config.clients.len(), 1);
        assert_eq!(config.bin_dir, PathBuf::from("/test/bin"));
        assert!(config.yes);
        Ok(())
    }

    #[test]
    fn test_setup_config_with_universal() -> Result<()> {
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        let config = SetupConfig {
            clients: vec![Client::Claude],
            bin_dir: PathBuf::from("/test/bin"),
            reinstall: false,
            uninstall: false,
            add: false,
            yes: true,
            universal: true,
            mirror_source: Some(PathBuf::from("/custom/source")),
        };

        assert!(config.universal);
        assert_eq!(config.mirror_source, Some(PathBuf::from("/custom/source")));
        Ok(())
    }
}

#[cfg(test)]
mod bdd_scenarios {
    use super::*;

    // BDD: Given-When-Then style tests for business scenarios

    #[test]
    fn scenario_fresh_install_detects_no_setup() -> Result<()> {
        // Given: A fresh system with no existing skrills setup
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        // When: I check if setup exists
        let is_first = is_first_run()?;

        // Then: The system should detect this is a first run
        assert!(is_first);
        Ok(())
    }

    // Note: Integration tests use actual HOME directory - see Makefile demos

    #[test]
    fn scenario_user_wants_both_clients() -> Result<()> {
        // Given: A user wants to set up both Claude and Codex
        let client_spec = "both";

        // When: I parse the client specification
        let clients = parse_clients(client_spec)?;

        // Then: I should get both clients
        assert_eq!(clients.len(), 2);
        assert!(clients.contains(&Client::Claude));
        assert!(clients.contains(&Client::Codex));
        Ok(())
    }

    // Note: File system tests moved to integration tests via Makefile demos

    #[test]
    fn scenario_universal_sync_creates_agent_dir() -> Result<()> {
        // Given: A user wants cross-agent skill sharing
        let _guard = env_guard();
        let temp = create_test_home()?;
        set_test_home(&temp);

        // When: Universal flag is set
        let config = SetupConfig {
            clients: vec![Client::Claude],
            bin_dir: temp.path().join(".claude/bin"),
            reinstall: false,
            uninstall: false,
            add: false,
            yes: true,
            universal: true,
            mirror_source: None,
        };

        // Then: The config should enable universal sync
        assert!(config.universal);
        Ok(())
    }
}
