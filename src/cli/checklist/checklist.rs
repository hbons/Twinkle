//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::app::App;
use crate::cli::util::*;

use super::checklist_config::*;
use super::checklist_platform::*;
use super::checklist_repository::*;
use super::checklist_ssh::*;


pub enum Check {
    Fail(Option<String>),
    Missing, //
    Pass(Option<String>),
    // Invalid, //
    // Error, TODO: instead of Result
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass(_) => write!(f, "✓"),
            Self::Missing => write!(f, "?"),
            Self::Fail(_) => write!(f, "!"),
        }
    }
}


impl App {
    pub fn cli_command_checklist(
        &mut self,
        args: &Vec<String>,
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        self.run_checklist(&path)
    }


    fn run_checklist(
        &self,
        path: &Path,
    ) -> Result<(), Box<dyn Error>>
    {
        print_header("Platform");
        run_check("Supported OS", &is_supported_os, &path);
        run_check("Supported ARCH", &is_supported_arch, &path);

        print_header("Dependencies");
        run_check("OpenSSH", &is_openssh_installed, &path);
        run_check("Git", &is_git_installed, &path);
        run_check("Git LFS", &is_git_lfs_installed, &path);

        print_header("Secure Shell");
        run_check("ssh-agent running", &is_ssh_agent_running, &path);
        run_check("Keys added to agent", &is_key_added_to_agent, &path);

        print_header("Connectivity");
        run_check("Host reachable", &is_host_reachable, &path);
        // run_check("Host known", &is_host_known, &path);
        run_check("Host uses SSH", &is_host_using_ssh, &path);
        run_check("Host supports ED25519", &is_host_supporting_ed25519, &path);
        run_check("Host supports ECDSA", &is_host_supporting_ecdsa, &path);
        run_check("Host supports RSA", &is_host_supporting_rsa, &path);
        run_check("Host knows client SSH key", &is_client_key_known_to_host, &path);

        print_header("Repository");
        run_check(".git/ present", &is_git_dir_present, &path);
        run_check(".git/info/exclude valid", &is_git_info_exclude_valid, &path);
        run_check(".git/info/attributes valid", &is_git_info_attributes_valid, &path);
        run_check("On a branch", &is_git_on_a_branch, &path); // TODO: name branch
        run_check("Not in a merge", &is_git_not_in_a_merge, &path);
        run_check("Files treated as binary", &is_git_attributes_all_binary, &path); // TODO: * merge=binary

        print_header("Git Config");
        run_check(".git/config valid", &is_git_config_valid, &path);
        run_check("remote.origin.url", &is_git_remote_url_valid, &path);
        run_check("submodule.recurse", &is_git_ignoring_submodules, &path); // TODO: green ": true"
        run_check("core.attributesFile = \"\"", &is_git_ignoring_submodules, &path);
        run_check("core.excludesFile = \"\"", &is_git_ignoring_submodules, &path);
        // ("core.autocrlf", "input"), // Text files will keep original line endings when checked out, CRLF chars are normalized to LF when committed
        // ("core.fileMode", "false"), // Ignore permission changes
        // ("core.ignoreCase", "false"), // Be case sensitive explicitly to work on Mac
        // ("core.precomposeUnicode", "true"), // Use the same Unicode form on all filesystems
        // ("core.quotePath", "false"), // Output Unicode characters: '"h\303\251"' becomes 'hé'
        // ("core.safecrlf", "false"),
        // ("push.default", "current"), // Push only current branch to matching remote
        run_check("user.name", &is_git_user_name_set, &path);
        run_check("user.email", &is_git_user_email_set, &path);

        print_header(&format!("{} Config", self.name));
        run_check(".twinkle/config valid", &is_git_user_email_set, &path);
        run_check("twinkle.enabled", &is_git_user_email_set, &path);
        run_check("twinkle.lfs.enabled", &is_git_user_email_set, &path);
        run_check("twinkle.lfs.sizeThreshold", &is_git_user_email_set, &path);
        run_check("twinkle.notify.enabled", &is_git_user_email_set, &path);
        run_check("twinkle.notify.url", &is_git_user_email_set, &path);

        println!();
        print_legend();
        println!();

        Ok(())
    }
}


pub fn run_check(
    title: &str,
    check: &dyn Fn(&Path)  -> Result<Check, Box<dyn Error>>,
    path: &Path, // TODO: use current_dir?
) {
    match check(path) {
        Ok(check) =>
            match check {
                Check::Pass(Some(ref s)) => println!("    {} {title}: \x1b[32m{}\x1b[0m", cli_green(&check.to_string()), cli_green(s)),
                Check::Fail(Some(ref s)) => println!("    \x1b[31m{}\x1b[0m {title}: \x1b[31m{}\x1b[0m", cli_red(&check.to_string()), cli_red(s)),
                Check::Pass(None) =>        println!("    \x1b[32m{}\x1b[0m {title}", cli_green(&check.to_string())),
                Check::Fail(None) =>        println!("    \x1b[31m{}\x1b[0m {title}", cli_red(&check.to_string())),
                Check::Missing =>           println!("    \x1b[33m{}\x1b[0m {title}: \x1b[33m{}\x1b[0m", cli_yellow(&check.to_string()), cli_yellow("Missing")),
            },
        _ => println!("    \x1b[31m?\x1b[0m {title}: \x1b[33mCheck Failed\x1b[0m"),
    };
}


fn print_header(s: &str) {
    println!("\n  {}\n", cli_bold(s));
}

fn print_legend() {
    println!(
        "  {} {}",
        cli_yellow(&Check::Missing.to_string()),
        cli_dimmed("= Check failed but should not disrupt sync"),
    );
    println!(
        "  {} {}",
        cli_red(&Check::Fail(None).to_string()),
        cli_dimmed("= Check failed and could disrupt sync"),
    );
}
