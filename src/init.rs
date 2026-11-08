use crate::embed::{self, Modules, Template};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub struct InitOptions {
    pub target: PathBuf,
    pub name: Option<String>,
    pub prefixes: String,
    pub verify: String,
    pub with: Option<String>,
}

pub fn run(opts: InitOptions) -> Result<(), Box<dyn std::error::Error>> {
    let target_arg = &opts.target;
    if target_arg.exists() && !target_arg.is_dir() {
        return Err(format!(
            "ledgibly: {} exists and is not a directory",
            target_arg.display()
        )
        .into());
    }
    fs::create_dir_all(target_arg)?;
    let target = fs::canonicalize(target_arg)?;

    let name = opts.name.unwrap_or_else(|| {
        target
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "project".to_string())
    });

    let prefixes = ensure_art_prefix(&opts.prefixes);
    let modules = parse_modules(opts.with.as_deref())?;

    let mut existing = Vec::new();
    collect_conflicts(&target, template_rel_paths(), &mut existing);
    for mod_name in &modules {
        collect_conflicts(&target, module_rel_paths(mod_name), &mut existing);
    }
    if modules.iter().any(|m| m == "claude") {
        let claude_link = target.join(".claude/skills");
        if path_exists_or_symlink(&claude_link) {
            existing.push(claude_link);
        }
    }

    if !existing.is_empty() {
        eprintln!("ledgibly: refusing to overwrite:");
        for p in &existing {
            eprintln!("  {}", p.display());
        }
        return Err("refusing to overwrite".into());
    }

    let mut copied: Vec<PathBuf> = Vec::new();
    write_template(&target, &mut copied)?;
    for mod_name in &modules {
        write_module(&target, mod_name, &mut copied)?;
    }

    let fragments = load_fragments(&modules);
    if !fragments.is_empty() {
        splice_fragments(&target.join("AGENTS.md"), &fragments)?;
    }

    let date = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    // Rfc3339 may include fractional seconds; match bash `date -u +%Y-%m-%dT%H:%M:%SZ`.
    let date = truncate_rfc3339_seconds(&date);

    let repl = [
        ("{{PROJECT}}", name.as_str()),
        ("{{PREFIXES}}", prefixes.as_str()),
        ("{{VERIFY}}", opts.verify.as_str()),
        ("{{DATE}}", date.as_str()),
    ];
    for path in &copied {
        replace_placeholders(path, &repl)?;
    }

    append_docs_index(&target)?;
    make_checker_executable(&target.join("scripts/ledger_check.py"))?;
    link_hub_skills(&target, modules.iter().any(|m| m == "claude"))?;

    println!("ledgibly: created {} ({})", target.display(), name);
    println!("ledgibly: remaining <fill me> lines:");
    print_fill_me(&target)?;

    Ok(())
}

pub fn ensure_art_prefix(prefixes: &str) -> String {
    let parts: Vec<&str> = prefixes.split_whitespace().collect();
    if parts.contains(&"ART") {
        parts.join(" ")
    } else {
        let mut out = Vec::with_capacity(parts.len() + 1);
        out.push("ART");
        out.extend(parts);
        out.join(" ")
    }
}

fn truncate_rfc3339_seconds(s: &str) -> String {
    // 2026-11-08T07:13:46.123456789Z -> 2026-11-08T07:13:46Z
    if let Some(dot) = s.find('.') {
        if s.ends_with('Z') {
            return format!("{}Z", &s[..dot]);
        }
    }
    s.to_string()
}

fn parse_modules(with: Option<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let Some(raw) = with else {
        return Ok(Vec::new());
    };
    let valid = embed::module_names();
    let mut out = Vec::new();
    for part in raw.split(',') {
        let mod_name = part.trim();
        if mod_name.is_empty() {
            continue;
        }
        if !valid.contains(mod_name) {
            let list: Vec<_> = valid.iter().cloned().collect();
            return Err(format!(
                "ledgibly: unknown module {mod_name} (valid: {})",
                list.join(" ")
            )
            .into());
        }
        out.push(mod_name.to_string());
    }
    Ok(out)
}

fn template_rel_paths() -> Vec<String> {
    Template::iter()
        .map(|p| p.as_ref().to_string())
        .filter(|p| !embed::skip_copy_name(p))
        .collect()
}

fn module_rel_paths(mod_name: &str) -> Vec<String> {
    let prefix = format!("{mod_name}/");
    Modules::iter()
        .filter_map(|p| {
            let s = p.as_ref();
            if !s.starts_with(&prefix) {
                return None;
            }
            let rel = &s[prefix.len()..];
            if embed::skip_copy_name(rel) || embed::skip_copy_name(s) {
                return None;
            }
            Some(rel.to_string())
        })
        .collect()
}

fn collect_conflicts(target: &Path, rels: Vec<String>, existing: &mut Vec<PathBuf>) {
    for rel in rels {
        let dest = target.join(&rel);
        if path_exists_or_symlink(&dest) {
            existing.push(dest);
        }
    }
}

fn path_exists_or_symlink(path: &Path) -> bool {
    path.symlink_metadata().is_ok()
}

fn write_bytes(dest: &Path, data: &[u8], copied: &mut Vec<PathBuf>) -> io::Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(dest, data)?;
    copied.push(dest.to_path_buf());
    Ok(())
}

fn write_template(
    target: &Path,
    copied: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in Template::iter() {
        let rel = path.as_ref();
        if embed::skip_copy_name(rel) {
            continue;
        }
        let file = Template::get(rel).ok_or_else(|| format!("missing embed {rel}"))?;
        write_bytes(&target.join(rel), &file.data, copied)?;
    }
    Ok(())
}

fn write_module(
    target: &Path,
    mod_name: &str,
    copied: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let prefix = format!("{mod_name}/");
    for path in Modules::iter() {
        let s = path.as_ref();
        if !s.starts_with(&prefix) {
            continue;
        }
        if embed::skip_copy_name(s) {
            continue;
        }
        let rel = &s[prefix.len()..];
        let file = Modules::get(s).ok_or_else(|| format!("missing embed {s}"))?;
        write_bytes(&target.join(rel), &file.data, copied)?;
    }
    Ok(())
}

fn load_fragments(modules: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for mod_name in modules {
        let key = format!("{mod_name}/AGENTS.md.fragment");
        if let Some(file) = Modules::get(&key) {
            let text = String::from_utf8_lossy(&file.data);
            let mut block = text.trim_end().to_string();
            block.push('\n');
            out.push(block);
        }
    }
    out
}

fn splice_fragments(agents: &Path, fragments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let marker = "<!-- end:agent-profile -->";
    let mut text = fs::read_to_string(agents)?;
    let block = fragments.join("\n");
    if text.contains(marker) {
        text = text.replacen(marker, &format!("{block}\n{marker}"), 1);
    } else {
        text = format!("{}\n\n{block}", text.trim_end());
    }
    fs::write(agents, text)?;
    Ok(())
}

fn replace_placeholders(
    path: &Path,
    repl: &[(&str, &str)],
) -> Result<(), Box<dyn std::error::Error>> {
    let meta = match path.symlink_metadata() {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };
    if meta.file_type().is_symlink() || !meta.is_file() {
        return Ok(());
    }
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Ok(()),
    };
    let Ok(mut text) = String::from_utf8(bytes) else {
        return Ok(());
    };
    if !text.contains("{{") {
        return Ok(());
    }
    for (key, value) in repl {
        text = text.replace(key, value);
    }
    fs::write(path, text)?;
    Ok(())
}

fn append_docs_index(target: &Path) -> io::Result<()> {
    let docs_readme = target.join("docs/README.md");
    if !docs_readme.is_file() {
        return Ok(());
    }
    let mut text = fs::read_to_string(&docs_readme)?;
    if target.join("docs/ui").is_dir() && !text.contains("ui/README") {
        text.push_str("\nUI inventory: [docs/ui](ui/README.md).\n");
        fs::write(&docs_readme, &text)?;
    }
    let text = fs::read_to_string(&docs_readme)?;
    if target.join("docs/sources").is_dir() && !text.contains("sources/README") {
        let mut text = text;
        text.push_str("\nCanonical sources: [docs/sources](sources/README.md).\n");
        fs::write(&docs_readme, text)?;
    }
    Ok(())
}

fn make_checker_executable(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if path.is_file() {
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(path, perms)?;
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

fn link_hub_skills(target: &Path, also_claude: bool) -> Result<(), Box<dyn std::error::Error>> {
    let agents = target.join(".agents/skills");
    fs::create_dir_all(agents.parent().unwrap_or(target))?;
    fs::create_dir_all(&agents)?;
    if also_claude {
        symlink_to_hub(&target.join(".claude/skills"), &agents)?;
    }
    Ok(())
}

fn symlink_to_hub(link: &Path, agents: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = link.parent() {
        fs::create_dir_all(parent)?;
    }

    #[cfg(unix)]
    {
        let _ = agents;
        std::os::unix::fs::symlink("../.agents/skills", link)?;
    }

    #[cfg(windows)]
    {
        match std::os::windows::fs::symlink_dir("../.agents/skills", link) {
            Ok(()) => {}
            Err(e) => {
                eprintln!(
                    "ledgibly: warning: could not symlink {} ({e}); copying instead",
                    link.display()
                );
                copy_dir_recursive(agents, link)?;
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        copy_dir_recursive(agents, link)?;
    }

    Ok(())
}

#[cfg(any(windows, not(any(unix, windows))))]
fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dest.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

fn print_fill_me(target: &Path) -> io::Result<()> {
    let mut paths = Vec::new();
    collect_md_mdc(target, &mut paths)?;
    paths.sort();
    for path in paths {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if line.contains("<fill me>") {
                let rel = path.strip_prefix(target).unwrap_or(&path);
                println!("{}:{}:{line}", rel.display(), i + 1);
            }
        }
    }
    Ok(())
}

fn collect_md_mdc(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ty = entry.file_type()?;
        if ty.is_dir() {
            collect_md_mdc(&path, out)?;
        } else if ty.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "md" || ext == "mdc" {
                    out.push(path);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn art_prepended_when_missing() {
        assert_eq!(ensure_art_prefix("FE BE"), "ART FE BE");
        assert_eq!(ensure_art_prefix("ART DOC QA"), "ART DOC QA");
    }

    #[test]
    fn date_truncation() {
        assert_eq!(
            truncate_rfc3339_seconds("2026-11-08T07:13:46.123456789Z"),
            "2026-11-08T07:13:46Z"
        );
        assert_eq!(
            truncate_rfc3339_seconds("2026-11-08T07:13:46Z"),
            "2026-11-08T07:13:46Z"
        );
    }

    #[test]
    fn parse_unknown_module_errors() {
        let err = parse_modules(Some("nope")).unwrap_err().to_string();
        assert!(err.contains("unknown module"));
    }

    #[test]
    fn valid_modules_from_embed() {
        let names = embed::module_names();
        assert!(names.contains("figma"));
        assert!(names.contains("claude"));
        assert!(names.contains("copilot"));
    }
}
