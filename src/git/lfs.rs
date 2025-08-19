//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::fs::{ self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{ Path, PathBuf };

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-lfs.com/
    //       https://github.com/git-lfs/git-lfs/security

    /// Configures the LFS filters with GIT_SSH_COMMAND
    pub fn lfs_config_filters(&self) -> Result<(), Box<dyn Error>> {
        let smudge = "git-lfs smudge -- %f";
        let smudge = &format!("env GIT_SSH_COMMAND='{}' {}", self.GIT_SSH_COMMAND, smudge);

        self.config_set("filter.lfs.smudge",   smudge)?; // Runs on file commit
        self.config_set("filter.lfs.clean",    "git-lfs clean -- %f")?; // Runs on file checkout
        self.config_set("filter.lfs.process",  "git-lfs filter-process")?; // Prevents spawning many threads
        self.config_set("filter.lfs.required", "true")?;

        Ok(())
    }


    /// Overwrites the pre-push hook and sets GIT_SSH_COMMAND
    pub fn lfs_install_pre_push_hook(&self) -> Result<(), Box<dyn Error>> {
        let git_lfs_path = Path::new("git-lfs");
        let hook_path = self.working_dir.join(".git/hooks/pre-push");

        if let Some(hook_dir) = hook_path.parent() {
            if !hook_dir.try_exists()? {
                fs::create_dir_all(hook_dir)?;
            }
        }

        let hook = format!(
            "#!/bin/sh\n\
            env GIT_SSH_COMMAND='{}' {} pre-push \"$@\"", // $@ passes all args along
            self.GIT_SSH_COMMAND, git_lfs_path.display()
        );

        let user_rwx = Permissions::from_mode(0o700);
        fs::write(&hook_path, hook)?;
        fs::set_permissions(&hook_path, user_rwx)?;

        Ok(())
    }
}


impl GitEnvironment {
    /// Fetching Git LFS objects separately benefits from concurrency
    pub fn lfs_fetch(&self) -> Result<(), Box<dyn Error>> {
        self.run("lfs", &["fetch"])?;
        Ok(())
    }


    /// Looks at .gitattributes and committed/staged pointer files
    pub fn lfs_ls_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let output = self.run("lfs", &["ls-files", "--name-only"])?;

        let files = output.stdout.lines()
            .map(PathBuf::from)
            .collect();

        Ok(files)
    }


    /// Puts the path in .gitattributes under LFS filters
    pub fn lfs_track(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        if path.file_name() == Some(OsStr::new(".gitattributes")) {
            return Err("Cannot track .gitattributes".into());
        }

        self.run("lfs", &["track", &path.to_string_lossy()])?;
        Ok(())
    }


    pub fn lfs_version(&self) -> String {
        match self.run("lfs", &["--version"]) {
            Ok(output) => output.stdout.trim().into(),
            Err(_) => "\x1b[33mGit LFS not found\x1b[0m".into(),
        }
    }
}
