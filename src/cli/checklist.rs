//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env::{ self, consts::OS };
use std::fmt;
use std::path::Path;
use std::process::{ Command, Stdio };

use crate::app::App;
use crate::git::objects::environment::GitEnvironment;
use super::util::*;


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
        self.run_check("Supported OS", &is_supported_os, &path);

        print_header("Dependencies");
        self.run_check("OpenSSH", &is_openssh_installed, &path);
        self.run_check("Git", &is_git_installed, &path);
        self.run_check("Git LFS", &is_git_lfs_installed, &path);

        print_header("Secure Shell");
        self.run_check("ssh-agent running", &is_ssh_agent_running, &path);
        self.run_check("Keys added to agent", &is_key_added_to_agent, &path);

        print_header("Connectivity");
        self.run_check("Host reachable", &is_host_reachable, &path);
        // self.run_check("Host uses SSH", &is_host_using_ssh, &path);
        // self.run_check("Host supports ED25519", &is_host_supporting_ed25519, &path);
        // self.run_check("Host supports ECDSA", &is_host_supporting_ecdsa, &path);
        // self.run_check("Host supports RSA", &is_host_supporting_rsa, &path);
        // self.run_check("Host knows client SSH key", &is_client_key_known_to_host, &path);

        // print_header("Repository");
        // self.run_check(".git directory present", &is_git_dir_present, &path);
        // self.run_check(".git/config valid", &is_git_config_valid, &path);
        // self.run_check(".git/config/exclude valid", &is_git_config_exclude_valid, &path);
        // self.run_check(".git/config/attributes valid", &is_git_config_attributes_valid, &path);
        // self.run_check("On a branch", &is_git_on_a_branch, &path);
        // self.run_check("Remote origin URL valid", &is_git_remote_url_valid, &path);
        // self.run_check("User name set", &is_git_user_name_set, &path);
        // self.run_check("User email set", &is_git_user_email_set, &path);
        // self.run_check("User signing key set", &is_git_user_signing_key_set, &path);
        // self.run_check("Commit signing enabled", &is_git_commit_signing_enabled, &path);
        // self.run_check("Files treated as binary", &is_git_attributes_all_binary, &path);

        // print_header("Twinkle");
        // self.run_check("Enabled", &is_twinkle_enabled, &path);
        // self.run_check(".twinkle/config valid", &is_twinkle_config_valid, &path);
        // self.run_check("Push notifications enabled", &is_twinkle_push_noticications_enabled, &path);
        // self.run_check("Git LFS enabled", &is_git_lfs_enabled, &path);
        // self.run_check("Git LFS size threshold set", &is_git_lfs_threshold_set, &path);

        // TODO: Check important git settings / git config --list --show-origin

        println!();
        Ok(())
    }


    fn run_check(&self,
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
                    Check::Missing =>           println!("    \x1b[31m{check}\x1b[0m {title}: \x1b[31mMissing\x1b[0m"),
                },
            _ => println!("    \x1b[31m?\x1b[0m {title}: \x1b[31mCheck Failed\x1b[0m"), // TODO: Orange
        };
    }
}


fn print_header(s: &str) {
    println!("\n  {}\n", cli_bold(s));
}


pub enum Check {
    Pass(Option<String>),
    Missing,
    Fail(Option<String>),
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass(_) => write!(f, "✓"),
            Self::Missing => write!(f, "~"),
            Self::Fail(_) => write!(f, "✗"),
        }
    }
}


// Platform

fn is_supported_os(_path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(
        match OS {
            s if s.ends_with("bsd") => Check::Pass(Some("*BSD".into())),
            "linux" => Check::Pass(Some("Linux".into())),
            "macos" => Check::Pass(Some("macOS".into())),
            "windows" => Check::Fail(Some("Windows".into())),
            _ => Check::Fail(Some("Unknown".into())),
        }
    )
}


// Dependencies

fn is_openssh_installed(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let ssh = Command::new("ssh").arg("-V").output();

    Ok(match ssh {
        Ok(o) => Check::Pass(Some(
            String::from_utf8_lossy(&o.stderr).trim().to_string())
        ),
        _ => Check::Missing,
    })
}

fn is_git_installed(path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(match GitEnvironment::new(path).version() {
        Some(s) => Check::Pass(Some(s.to_string())),
        _ => Check::Missing,
    })
}

fn is_git_lfs_installed(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);
    Ok(Check::Pass(Some(git.lfs_version().unwrap())))
}


// Secure Shell

fn is_ssh_agent_running(_path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(match env::var("SSH_AUTH_SOCK") {
        Ok(_) => Check::Pass(None),
        _ => Check::Fail(None),
    })
}

fn is_key_added_to_agent(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let ssh = Command::new("ssh-add")    .stdout(Stdio::null())
    .stderr(Stdio::null()).arg("-L").status();

    match ssh {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


// Connectivity

fn is_host_reachable(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let nc = Command::new("nc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-zv")
        .arg("1.1.1.1") // TODO: use remote.origin.url
        .arg("80")
        .status();

    match nc {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}
