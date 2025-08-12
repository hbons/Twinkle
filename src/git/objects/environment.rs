//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::process::Command;

use crate::log;

use super::output::GitOutput;


#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct GitEnvironment {
    pub working_dir:  PathBuf,

    pub HOME:   PathBuf,
    pub PREFIX: PathBuf,
    pub LANG:   String, // Still needed? Should be overridden by LC_ALL
    pub LC_ALL: String, // Override all locale settings

    pub GIT_CONFIG_GLOBAL:   String,
    pub GIT_CONFIG_SYSTEM:   String,
    pub GIT_CONFIG_NOSYSTEM: String,

    pub GIT_EXEC_PATH: PathBuf, // Location of `libexec/git-core/`
    pub GIT_PAGER: String,
    pub GIT_SSH_COMMAND: String,
    pub GIT_TERMINAL_PROMPT: String,
}


impl Default for GitEnvironment {
    fn default() -> Self {
        let working_dir  = Path::new(".").to_path_buf();

        GitEnvironment {
            working_dir,

            HOME: Path::new(".").to_path_buf(),
            PREFIX: Path::new(".").to_path_buf(), // Don't use the system gitconfig
            LANG:   "en_US.UTF8".to_string(), // Default to English for parsing errors/warnings
            LC_ALL: "en_US.UTF8".to_string(),

            GIT_CONFIG_GLOBAL:   "/dev/null".to_string(), // Don't use the system gitconfig
            GIT_CONFIG_SYSTEM:   "/dev/null".to_string(), // Don't use the system gitconfig
            GIT_CONFIG_NOSYSTEM: "1".to_string(), // Don't use the system gitconfig

            GIT_EXEC_PATH: Path::new("/app/share/libexec/git-core").to_path_buf(),
            GIT_PAGER: "".to_string(), // Don't use a pager for large output
            GIT_SSH_COMMAND: "ssh".to_string(), // Use a custom SSH command
            GIT_TERMINAL_PROMPT: "false".to_string(), // Don't hang on prompts
        }
    }
}


impl GitEnvironment {
    pub fn new(path: &Path) -> GitEnvironment {
        Self {
            working_dir: path.into(),
            ..Default::default()
        }
    }


    pub fn get_environment(&self) -> Vec<(String, String)> {
        vec![
            ("HOME".into(), self.HOME.to_string_lossy().into()),
            ("PREFIX".into(), self.PREFIX.to_string_lossy().into()),
            ("LANG".into(), self.LANG.clone()),
            ("LC_ALL".into(), self.LC_ALL.clone()),

            ("GIT_CONFIG_GLOBAL".into(),   self.GIT_CONFIG_GLOBAL.clone()),
            ("GIT_CONFIG_SYSTEM".into(),   self.GIT_CONFIG_SYSTEM.clone()),
            ("GIT_CONFIG_NOSYSTEM".into(), self.GIT_CONFIG_NOSYSTEM.clone()),

            ("GIT_EXEC_PATH".into(), self.GIT_EXEC_PATH.to_string_lossy().into()),
            ("GIT_PAGER".into(), self.GIT_PAGER.clone()),
            ("GIT_SSH_COMMAND".into(), self.GIT_SSH_COMMAND.clone()),
            ("GIT_TERMINAL_PROMPT".into(), self.GIT_TERMINAL_PROMPT.clone()),
        ]
    }
}


impl GitEnvironment {
    pub fn run(&self, command: &str, args: &[&str]) -> Result<GitOutput, Box<dyn Error>> {
        let env = Vec::new();
        self.run_with_env(command, args, env)
    }


    /// Runs with extra environment variables
    pub fn run_with_env(&self, command: &str, args: &[&str], env: Vec<(String, String)>) -> Result<GitOutput, Box<dyn Error>> {
        log::debug(&format!("git {} {}", command, args.join(" ")));

        let output = Command::new("git")
            .current_dir(&self.working_dir)
            // .env_clear() // TODO: PATH is missing and can't find git-lfs
            .envs(self.get_environment())
            .envs(env)
            .arg(command)
            .args(args)
            .output()?;

        let git_output = GitOutput {
            exit_code: output.status.code().unwrap_or(0),
            stdout: String::from_utf8_lossy(&output.stdout).trim_end().to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim_end().to_string(),
        };

        if output.status.success() {
            Ok(git_output)
        } else {
            log::error(&git_output.stderr);
            Err(format!("Could not run git-{}: {}", command, git_output.stderr).into())
        }
    }
}
