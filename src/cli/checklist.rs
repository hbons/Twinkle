//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::app::App;

use super::util::*;

// TODO: subdir
use super::checklist_repository::*;
use super::checklist_platform::*;
use super::checklist_ssh::*;
// use super::checklist_sync::*;


pub enum Check {
    Fail(Option<String>),
    Missing,
    Pass(Option<String>),
    // Error, TODO: instead of Result
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass(_) => write!(f, "✓"),
            Self::Missing => write!(f, "!"),
            Self::Fail(_) => write!(f, "✗"),
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


    fn run_checklist(&self, path: &Path) -> Result<(), Box<dyn Error>> {
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
        run_check(".git directory present", &is_git_dir_present, &path);
        run_check(".git/config valid", &is_git_config_valid, &path);
        run_check(".git/info/exclude valid", &is_git_info_exclude_valid, &path);
        run_check(".git/info/attributes valid", &is_git_info_attributes_valid, &path);
        run_check("On a branch", &is_git_on_a_branch, &path);
        run_check("Not in a merge", &is_git_not_in_a_merge, &path);
        run_check("remote.origin.url valid", &is_git_remote_url_valid, &path);
        run_check("user.name set", &is_git_user_name_set, &path);
        run_check("user.email set", &is_git_user_email_set, &path);
        run_check("user.signingKey set", &is_git_user_signing_key_set, &path);
        run_check("Commit signing enabled", &is_git_commit_signing_enabled, &path);
        run_check("Files treated as binary", &is_git_attributes_all_binary, &path);
        run_check("submodule.recurse", &is_git_ignoring_submodules, &path);

        print_header("Sync");
        // run_check("Enabled", &is_twinkle_enabled, &path);
        // run_check(".twinkle/config valid", &is_twinkle_config_valid, &path);
        // run_check("Git LFS enabled", &is_git_lfs_enabled, &path);
        // run_check("Git LFS size threshold set", &is_git_lfs_threshold_set, &path);
        // run_check("Push notifications enabled", &is_twinkle_push_noticications_enabled, &path);
        // run_check("Push notifications URL set", &is_twinkle_push_noticications_url_set, &path);

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
                Check::Pass(Some(ref s)) => println!("    \x1b[32m{check}\x1b[0m {title}: \x1b[32m{s}\x1b[0m"),
                Check::Fail(Some(ref s)) => println!("    \x1b[31m{check}\x1b[0m {title}: \x1b[31m{s}\x1b[0m"),
                Check::Pass(None) =>        println!("    \x1b[32m{check}\x1b[0m {title}"),
                Check::Fail(None) =>        println!("    \x1b[31m{check}\x1b[0m {title}"),
                Check::Missing =>           println!("    \x1b[33m{check}\x1b[0m {title}: \x1b[33mMissing\x1b[0m"),
            },
        _ => println!("    \x1b[31m?\x1b[0m {title}: \x1b[33mCheck Failed\x1b[0m"),
    };
}


fn green() -> String {"".into() } // TODO
fn red() -> String { "".into() }
fn yellow() -> String { "".into() }


fn print_header(s: &str) {
    println!("\n  {}\n", cli_bold(s));
}
