//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use crate::app::App;
use crate::cli::util::*;

use super::check_config::*;
use super::check_platform::*;
use super::check_repository::*;
use super::check_ssh::*;
use super::outcome::Outcome;


impl App {
    pub fn cli_command_check(
        &mut self,
        args: &Vec<String>,
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        self.run_checks(&path)
    }


    fn run_checks(
        &self,
        path: &Path,
    ) -> Result<(), Box<dyn Error>>
    {
        print_header("Platform");
        run_check("Supported OS", &is_supported_os, path);
        run_check("Supported architecture", &is_supported_arch, path);
        run_check("Supported runtime", &is_supported_runtime, path);

        print_header("Dependencies");
        run_check("OpenSSH", &is_openssh_installed, path);
        run_check("Git", &is_git_installed, path);
        run_check("Git LFS", &is_git_lfs_installed, path);

        print_header("Secure Shell");
        run_check("ssh-agent running", &is_ssh_agent_running, path);
        run_check("ssh-agent has keys", &is_ssh_agent_has_keys, path);
        run_check("Host reachable", &is_ssh_host_reachable, path);
        run_check("Host sshd on port", &is_ssh_supported_host, path);
        run_check("Host key", &is_ssh_host_supporting_ed25519, path);
        run_check("Host key", &is_ssh_host_supporting_ecdsa, path);
        run_check("Host key", &is_ssh_host_supporting_rsa, path);
        run_check("Host knows client", &is_ssh_client_key_known_to_host, path);
        run_check("Client knows host", &is_ssh_host_known, path);

        print_header("Repository");
        run_check(".git/", &is_git_dir_present, path);
        run_check(".git/info/exclude", &is_git_info_exclude_exists, path);
        run_check(".git/info/attributes", &is_git_info_attributes_exists, path);
        run_check("On a branch", &is_git_on_a_branch, path);
        run_check("Not in a merge", &is_git_not_in_a_merge, path);
        run_check("On a branch", &is_git_on_a_branch, path);

        print_header("Config");
        run_check(".git/config", &is_git_config_valid, path);
        run_check("remote.origin.url", &is_git_remote_url_valid, path);
        run_check("core.attributesFile", &is_git_core_attributes_file_set, path);
        run_check("core.excludesFile", &is_git_core_excludes_file_set, path);
        run_check("push.default", &is_git_push_default_set, path);
        run_check("submodule.recurse", &is_git_submodule_recurse_set, path);
        run_check("user.name", &is_git_user_name_set, path);
        run_check("user.email", &is_git_user_email_set, path);

        // TODO:
        // ("core.autocrlf", "input"),
        // ("core.fileMode", "false"),
        // ("core.ignoreCase", "false"),
        // ("core.precomposeUnicode", "true"),
        // ("core.quotePath", "false"),
        // ("core.safecrlf", "false"),

        print_header("Sync");
        run_check(".twinkle/config", &is_twinkle_config_valid, path);
        run_check("twinkle.id", &is_twinkle_id_set, path);
        run_check("twinkle.enabled", &is_twinkle_enabled_set, path);
        run_check("twinkle.lfs.enabled", &is_twinkle_lfs_enabled_set, path);
        run_check("twinkle.push.enabled", &is_twinkle_push_enabled_set, path);

        // TODO: Find all git config options/filters/hooks that may have been added by the user

        print_legend();
        Ok(())
    }
}


type Check = dyn Fn(&Path) -> Outcome;

pub fn run_check(
    title: &str,
    check: &Check,
    path: &Path,
) {
    let outcome = check(path);

    match outcome {
        Outcome::Pass(Some(ref s)) =>
            println!(
                "    {} {title}: {}",
                cli_green(&outcome.to_string()),
                cli_green(s),
            ),
        Outcome::Fail(Some(ref s)) =>
            println!(
                "    {} {title}: {}",
                cli_red(&outcome.to_string()),
                cli_red(s),
            ),
        Outcome::Pass(None) =>
            println!(
                "    {} {title}",
                cli_green(&outcome.to_string()),
            ),
        Outcome::Fail(None) =>
            println!(
                "    {} {title}",
                cli_red(&outcome.to_string()),
            ),
        Outcome::Missing =>
            println!(
                "    {} {title}: {}",
                cli_yellow(&outcome.to_string()),
                cli_yellow("missing"),
            ),
        Outcome::Error =>
            println!(
                "    {} {title}: {}",
                cli_red(&Outcome::Fail(None).to_string()),
                cli_red("check failed"),
            ),
    }
}


fn print_header(s: &str) {
    println!("\n  {}\n", cli_bold(s));
}

fn print_legend() {
    println!();
    println!(
        "  {} {}",
        cli_yellow(&Outcome::Missing.to_string()),
        cli_dimmed("= Failed but should not disrupt sync"),
    );
    println!(
        "  {} {}",
        cli_red(&Outcome::Fail(None).to_string()),
        cli_dimmed("= Failed and could disrupt sync"),
    );
    println!();
}
