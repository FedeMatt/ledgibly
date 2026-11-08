use crate::embed::{self, Skills};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub struct InstallSkillsOptions {
    pub force: bool,
    /// Also copy into `~/.claude/skills` (Claude Code does not read the hub).
    pub claude: bool,
    /// Override home for tests. Defaults to `dirs`-less `HOME` / `USERPROFILE`.
    pub home: Option<PathBuf>,
}

pub fn run(opts: InstallSkillsOptions) -> Result<(), Box<dyn std::error::Error>> {
    let home = match opts.home {
        Some(h) => h,
        None => home_dir().ok_or("ledgibly: cannot determine home directory")?,
    };

    let ts = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "19700101T000000Z".to_string());
    let ts = backup_timestamp(&ts);

    let skill_names: Vec<String> = embed::skill_names().into_iter().collect();
    if skill_names.is_empty() {
        return Err("ledgibly: no skills embedded".into());
    }

    let mut dest_roots = vec![home.join(".agents/skills")];
    if opts.claude {
        dest_roots.push(home.join(".claude/skills"));
    }

    for name in &skill_names {
        let files = skill_files(name);
        if files.is_empty() {
            return Err(format!("ledgibly: missing embedded skill {name}").into());
        }
        for dest_root in &dest_roots {
            fs::create_dir_all(dest_root)?;
            install_one(name, &files, dest_root, &home, &ts, opts.force)?;
        }
        retire_leftover_cursor(&home, name, &ts)?;
    }

    println!("ledgibly: install-skills done (copy)");
    Ok(())
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(h) = std::env::var("HOME") {
        if !h.is_empty() {
            return Some(PathBuf::from(h));
        }
    }
    if let Ok(h) = std::env::var("USERPROFILE") {
        if !h.is_empty() {
            return Some(PathBuf::from(h));
        }
    }
    None
}

fn backup_timestamp(rfc3339: &str) -> String {
    // 2026-11-08T07:13:46.123Z -> 20261108T071346Z
    let s = if let Some(dot) = rfc3339.find('.') {
        if rfc3339.ends_with('Z') {
            format!("{}Z", &rfc3339[..dot])
        } else {
            rfc3339.to_string()
        }
    } else {
        rfc3339.to_string()
    };
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

fn skill_files(name: &str) -> BTreeMap<String, Vec<u8>> {
    let prefix = format!("{name}/");
    let mut map = BTreeMap::new();
    for path in Skills::iter() {
        let s = path.as_ref();
        if !s.starts_with(&prefix) {
            continue;
        }
        if embed::skip_copy_name(s) {
            continue;
        }
        let rel = s[prefix.len()..].to_string();
        if let Some(file) = Skills::get(s) {
            map.insert(rel, file.data.into_owned());
        }
    }
    map
}

fn kit_backup_dir(home: &Path, ts: &str, label: &str) -> PathBuf {
    home.join(".ledgibly/backups").join(ts).join(label)
}

fn dest_backup_label(dest_root: &Path) -> String {
    dest_root
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("skills")
        .trim_start_matches('.')
        .to_string()
}

fn install_one(
    name: &str,
    files: &BTreeMap<String, Vec<u8>>,
    dest_root: &Path,
    home: &Path,
    ts: &str,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest = dest_root.join(name);
    if path_exists_or_symlink(&dest) {
        if !force && contents_match(&dest, files)? {
            println!("already installed: {}", dest.display());
            return Ok(());
        }
        let bak = kit_backup_dir(home, ts, &dest_backup_label(dest_root));
        move_aside(&dest, &bak, name)?;
    }
    write_skill_tree(&dest, files)?;
    println!("copied {name} -> {}", dest.display());
    Ok(())
}

fn retire_leftover_cursor(
    home: &Path,
    name: &str,
    ts: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cursor_root = home.join(".cursor/skills");
    let dest = cursor_root.join(name);
    if !path_exists_or_symlink(&dest) {
        return Ok(());
    }
    let bak = kit_backup_dir(home, ts, "cursor");
    move_aside(&dest, &bak, name)?;
    Ok(())
}

fn move_aside(src: &Path, bak_dir: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(bak_dir)?;
    let bak_dest = bak_dir.join(name);
    if fs::rename(src, &bak_dest).is_err() {
        copy_path_recursive(src, &bak_dest)?;
        remove_path(src)?;
    }
    println!("backed up {} -> {}", src.display(), bak_dest.display());
    Ok(())
}

fn path_exists_or_symlink(path: &Path) -> bool {
    path.symlink_metadata().is_ok()
}

fn contents_match(dest: &Path, files: &BTreeMap<String, Vec<u8>>) -> io::Result<bool> {
    if dest.is_symlink() {
        return Ok(false);
    }
    if !dest.is_dir() {
        return Ok(false);
    }
    let mut on_disk: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    collect_files(dest, dest, &mut on_disk)?;
    Ok(on_disk == *files)
}

fn collect_files(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ty = entry.file_type()?;
        if ty.is_symlink() {
            return Ok(()); // treat as mismatch via caller comparing maps
        }
        if ty.is_dir() {
            collect_files(root, &path, out)?;
        } else if ty.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(rel, fs::read(&path)?);
        }
    }
    Ok(())
}

fn write_skill_tree(dest: &Path, files: &BTreeMap<String, Vec<u8>>) -> io::Result<()> {
    for (rel, data) in files {
        let path = dest.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, data)?;
    }
    Ok(())
}

fn copy_path_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let to = dest.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_path_recursive(&entry.path(), &to)?;
            } else {
                fs::copy(entry.path(), to)?;
            }
        }
    } else {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dest)?;
    }
    Ok(())
}

fn remove_path(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_timestamp_strips_punct() {
        assert_eq!(backup_timestamp("2026-11-08T07:13:46Z"), "20261108T071346Z");
    }

    #[test]
    fn skill_files_nonempty() {
        let files = skill_files("keep-implementation-ledger");
        assert!(files.contains_key("SKILL.md"));
        assert!(files.contains_key("reference.md"));
    }
}
