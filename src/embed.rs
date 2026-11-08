use rust_embed::Embed;
use std::collections::BTreeSet;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/template"]
#[exclude = ".DS_Store"]
#[exclude = "**/.DS_Store"]
pub struct Template;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/modules"]
#[exclude = ".DS_Store"]
#[exclude = "**/.DS_Store"]
pub struct Modules;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/skills"]
#[exclude = ".DS_Store"]
#[exclude = "**/.DS_Store"]
pub struct Skills;

pub fn skip_copy_name(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    name == "AGENTS.md.fragment" || name == ".DS_Store"
}

pub fn module_names() -> BTreeSet<String> {
    Modules::iter()
        .filter_map(|p| {
            let s = p.as_ref();
            let top = s.split('/').next()?;
            if top.is_empty() {
                None
            } else {
                Some(top.to_string())
            }
        })
        .collect()
}

pub fn skill_names() -> BTreeSet<String> {
    Skills::iter()
        .filter_map(|p| {
            let s = p.as_ref();
            let top = s.split('/').next()?;
            if top.is_empty() {
                None
            } else {
                Some(top.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_embeds_hidden_files() {
        assert!(
            Template::get(".cursor/rules/ponytail.mdc").is_some(),
            "missing .cursor/rules/ponytail.mdc"
        );
        assert!(Template::get(".gitignore").is_some(), "missing .gitignore");
        assert!(
            Template::get(".agents/skills/.gitkeep").is_some(),
            "missing .agents/skills/.gitkeep"
        );
    }

    #[test]
    fn modules_and_skills_are_nonempty() {
        let mods = module_names();
        assert!(mods.contains("figma"));
        assert!(mods.contains("rtk"));
        assert!(mods.contains("source-docs"));
        let skills = skill_names();
        assert!(skills.contains("keep-implementation-ledger"));
        assert!(skills.contains("code-review"));
        assert!(skills.contains("figma-ui"));
    }
}
