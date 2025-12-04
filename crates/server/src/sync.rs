//! Skill synchronization and AGENTS.md management.
//!
//! This module handles:
//! - Synchronizing skills from `~/.claude` to `~/.codex/skills-mirror`.
//! - Generating and updating `AGENTS.md` with available skills.

use anyhow::Result;
use skrills_discovery::{hash_file, SkillMeta};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

use crate::discovery::{
    collect_skills, is_skill_file, priority_labels, relative_path, AGENTS_SECTION_END,
    AGENTS_SECTION_START, AGENTS_TEXT,
};

/// Reports the outcome of a synchronization operation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct SyncReport {
    pub(crate) copied: usize,
    pub(crate) skipped: usize,
    /// Relative paths of skills that were copied (new or updated).
    pub(crate) copied_names: Vec<String>,
}

/// Synchronizes skills from Claude's directory to a mirror directory.
///
/// Walks through the source directory and copies `SKILL.md` files to the destination,
/// only copying if the file is new or has changed (based on hash comparison).
pub(crate) fn sync_from_claude(claude_root: &Path, mirror_root: &Path) -> Result<SyncReport> {
    let mut report = SyncReport::default();
    if !claude_root.exists() {
        return Ok(report);
    }
    for entry in WalkDir::new(claude_root)
        .min_depth(1)
        .max_depth(6)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !is_skill_file(&entry) {
            continue;
        }
        let src = entry.into_path();
        let rel = relative_path(claude_root, &src).unwrap_or_else(|| src.clone());
        let dest = mirror_root.join(rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let should_copy = if dest.exists() {
            hash_file(&dest)? != hash_file(&src)?
        } else {
            true
        };
        if should_copy {
            fs::copy(&src, &dest)?;
            report.copied += 1;
            // Store the relative path (directory name) for display
            if let Some(rel_path) = relative_path(claude_root, &src) {
                // Extract parent directory name as the skill name (e.g., "nested" from "nested/SKILL.md")
                let skill_name = rel_path
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or_else(|| rel_path.to_str().unwrap_or("unknown"));
                report.copied_names.push(skill_name.to_string());
            }
        } else {
            report.skipped += 1;
        }
    }
    Ok(report)
}

/// Renders skills as an XML manifest with priority rankings.
///
/// Generates an `<available_skills>` XML section including metadata about each skill:
/// source, location, path, and priority rank.
pub(crate) fn render_available_skills_xml(skills: &[SkillMeta]) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut out = String::from("<available_skills");
    out.push_str(&format!(" generated_at_utc=\"{}\"", ts));
    out.push_str(&format!(" priority=\"{}\"", priority_labels().join(",")));
    out.push_str(">\n");
    let priority_order = priority_labels();
    for s in skills {
        let rank = priority_order
            .iter()
            .position(|p| p == &s.source.label())
            .map(|i| i + 1)
            .unwrap_or(priority_order.len() + 1);
        out.push_str(&format!(
            "  <skill name=\"{}\" source=\"{}\" location=\"{}\" path=\"{}\" priority_rank=\"{}\" />\n",
            s.name,
            s.source.label(),
            s.source.location(),
            s.path.display(),
            rank
        ));
    }
    out.push_str("</available_skills>");
    out
}

/// Writes or updates the AGENTS.md file with current skills.
///
/// Discovers skills from the specified directories and updates the AGENTS.md file
/// with an XML manifest of available skills.
pub(crate) fn sync_agents(path: &Path, extra_dirs: &[PathBuf]) -> Result<()> {
    let skills = collect_skills(extra_dirs)?;
    sync_agents_with_skills(path, &skills)
}

/// Updates AGENTS.md with a specific set of skills.
///
/// Inserts a new `<available_skills>` section or replaces an existing one.
/// Creates the file with the default AGENTS.md template if it does not exist.
pub(crate) fn sync_agents_with_skills(path: &Path, skills: &[SkillMeta]) -> Result<()> {
    let xml = render_available_skills_xml(skills);
    let section = format!(
        "{start}\n{xml}\n{end}\n",
        start = AGENTS_SECTION_START,
        xml = xml,
        end = AGENTS_SECTION_END
    );

    let content = if path.exists() {
        let mut existing = fs::read_to_string(path)?;
        if let (Some(start), Some(end)) = (
            existing.find(AGENTS_SECTION_START),
            existing.find(AGENTS_SECTION_END),
        ) {
            let end_idx = end + AGENTS_SECTION_END.len();
            existing.replace_range(start..end_idx, &section);
            existing
        } else {
            format!("{existing}\n\n{section}")
        }
    } else {
        format!("{AGENTS_TEXT}\n\n{section}")
    };

    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use skrills_discovery::SkillSource;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn render_available_skills_xml_contains_location() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("codex/skills");
        fs::create_dir_all(&path).unwrap();
        let skill_path = path.join("alpha/SKILL.md");
        fs::create_dir_all(skill_path.parent().unwrap()).unwrap();
        fs::write(&skill_path, "hello").unwrap();
        let skills = vec![SkillMeta {
            name: "alpha/SKILL.md".into(),
            path: skill_path.clone(),
            source: SkillSource::Codex,
            root: path.clone(),
            hash: hash_file(&skill_path).unwrap(),
        }];
        let xml = render_available_skills_xml(&skills);
        assert!(xml.contains("location=\"global\""));
        assert!(xml.contains("alpha/SKILL.md"));
    }

    #[test]
    fn sync_agents_inserts_section() -> Result<()> {
        let tmp = tempdir()?;
        let agents = tmp.path().join("AGENTS.md");
        fs::write(&agents, "# Title")?;
        let skills = vec![SkillMeta {
            name: "alpha/SKILL.md".into(),
            path: tmp.path().join("alpha/SKILL.md"),
            source: SkillSource::Codex,
            root: tmp.path().join("codex/skills"),
            hash: "abc".into(),
        }];
        sync_agents_with_skills(&agents, &skills)?;
        let text = fs::read_to_string(&agents)?;
        assert!(text.contains(AGENTS_SECTION_START));
        assert!(text.contains("available_skills"));
        assert!(text.contains("location=\"global\""));
        assert!(text.contains(AGENTS_SECTION_END));
        assert!(text.contains("# Title"));
        Ok(())
    }

    #[test]
    fn sync_agents_sets_priority_rank_in_xml() -> Result<()> {
        let tmp = tempdir()?;
        let _agents = tmp.path().join("AGENTS.md");
        let skills = vec![SkillMeta {
            name: "alpha/SKILL.md".into(),
            path: tmp.path().join("alpha/SKILL.md"),
            source: SkillSource::Codex,
            root: tmp.path().join("codex/skills"),
            hash: "abc".into(),
        }];
        let xml = render_available_skills_xml(&skills);
        assert!(xml.contains("priority_rank=\"1\""));
        Ok(())
    }

    #[test]
    fn sync_from_claude_copies_and_updates() -> Result<()> {
        let tmp = tempdir()?;
        let claude_root = tmp.path().join("claude");
        let mirror_root = tmp.path().join("mirror");
        fs::create_dir_all(claude_root.join("nested"))?;
        let skill_src = claude_root.join("nested/SKILL.md");
        fs::write(&skill_src, "v1")?;

        let report1 = sync_from_claude(&claude_root, &mirror_root)?;
        assert_eq!(report1.copied, 1);
        let dest = mirror_root.join("nested/SKILL.md");
        assert_eq!(fs::read_to_string(&dest)?, "v1");

        std::thread::sleep(Duration::from_millis(5));
        fs::write(&skill_src, "v2")?;
        let report2 = sync_from_claude(&claude_root, &mirror_root)?;
        assert_eq!(report2.copied, 1);
        assert_eq!(fs::read_to_string(&dest)?, "v2");
        Ok(())
    }
}
