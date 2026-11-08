mod embed;
mod init;
mod skills;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "ledgibly",
    version,
    about = "Bootstrap coding-agent repos with an implementation ledger (Cursor default adapter)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Copy template/ and optional modules into a new repo
    Init {
        /// Target directory
        target: PathBuf,
        /// Project name (default: basename of target-dir)
        #[arg(long)]
        name: Option<String>,
        /// ID prefixes (default: ART DOC QA)
        #[arg(long, default_value = "ART DOC QA")]
        prefixes: String,
        /// Profile verify command
        #[arg(long, default_value = "python3 scripts/ledger_check.py")]
        verify: String,
        /// Modules: figma,source-docs,python-uv,formatter,database-design,rtk,claude,copilot
        #[arg(long)]
        with: Option<String>,
    },
    /// Install canonical skills into ~/.agents/skills, ~/.cursor/skills, and ~/.claude/skills
    #[command(name = "install-skills")]
    InstallSkills {
        /// Replace even when contents already match
        #[arg(long)]
        force: bool,
    },
}

fn main() -> ExitCode {
    if let Err(msg) = check_flag_values(std::env::args().skip(1)) {
        eprintln!("{msg}");
        return ExitCode::from(1);
    }

    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            // clap --help / --version exit through print; other errors to stderr.
            let _ = e.print();
            return if e.use_stderr() {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            };
        }
    };

    let result = match cli.command {
        Commands::Init {
            target,
            name,
            prefixes,
            verify,
            with,
        } => init::run(init::InitOptions {
            target,
            name,
            prefixes,
            verify,
            with,
        }),
        Commands::InstallSkills { force } => {
            skills::run(skills::InstallSkillsOptions { force, home: None })
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let msg = e.to_string();
            if msg != "refusing to overwrite" {
                eprintln!("{msg}");
            }
            ExitCode::from(1)
        }
    }
}

/// Match legacy init.sh: `--name` without a value prints `needs a value`.
fn check_flag_values(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let args: Vec<String> = args.into_iter().collect();
    let needs = ["--name", "--prefixes", "--verify", "--with"];
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if needs.contains(&a) {
            let next = args.get(i + 1).map(|s| s.as_str());
            if next.is_none() || next.is_some_and(|n| n.starts_with('-') && n != "-") {
                return Err(format!("ledgibly: {a} needs a value"));
            }
            i += 2;
            continue;
        }
        // --name=value form is fine for clap; no separate check.
        i += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_name_value() {
        let err = check_flag_values(["init".into(), "/tmp/x".into(), "--name".into()]).unwrap_err();
        assert!(err.contains("needs a value"));
    }
}
