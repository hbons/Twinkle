//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::fs::{ self, Permissions };
use std::os::unix::fs::PermissionsExt;
use std::path::{ Path, PathBuf };

use super::objects::environment::GitEnvironment;


pub const K_FILTER_LFS_SMUDGE: &str   = "filter.lfs.smudge";
pub const K_FILTER_LFS_CLEAN: &str    = "filter.lfs.clean";
pub const K_FILTER_LFS_PROCESS: &str  = "filter.lfs.process";
pub const K_FILTER_LFS_REQUIRED: &str = "filter.lfs.required";


impl GitEnvironment {
    // Docs: https://git-lfs.com/
    //       https://github.com/git-lfs/git-lfs/security

    /// Configures the LFS filters with GIT_SSH_COMMAND
    #[allow(non_snake_case)]
    pub fn lfs_config_filters(&self, GIT_SSH_COMMAND: Option<String>) -> Result<(), Box<dyn Error>> {
        let git_lfs = Path::new("git-lfs");

        let env = GIT_SSH_COMMAND
            .map(|v| format!("env GIT_SSH_COMMAND='{v}' ")) // Note ending space
            .unwrap_or_default();

        let smudge = &format!("{} smudge -- %f", git_lfs.display());
        let smudge = &format!("{}{}", env, smudge);

        let clean = &format!("{} clean -- %f", git_lfs.display());
        let filter_process = &format!("{} filter-process", git_lfs.display());

        self.config_set(K_FILTER_LFS_SMUDGE, smudge)?; // Runs on file commit
        self.config_set(K_FILTER_LFS_CLEAN, clean)?; // Runs on file checkout
        self.config_set(K_FILTER_LFS_PROCESS, filter_process)?; // Prevents spawning many threads
        self.config_set(K_FILTER_LFS_REQUIRED, &true.to_string())
    }


    /// Overwrites the pre-push hook and sets GIT_SSH_COMMAND
    #[allow(non_snake_case)]
    pub fn lfs_install_pre_push_hook(&self, GIT_SSH_COMMAND: Option<String>) -> Result<(), Box<dyn Error>> {
        let git_lfs = Path::new("git-lfs");
        let hook_path = self.working_dir.join(".git/hooks/pre-push");

        if let Some(hook_dir) = hook_path.parent() {
            if !hook_dir.try_exists()? {
                fs::create_dir_all(hook_dir)?;
            }
        }

        let env = GIT_SSH_COMMAND
            .map(|v| format!("env GIT_SSH_COMMAND='{v}' ")) // Note ending space
            .unwrap_or_default();

        let hook = format!(
            "#!/bin/sh\n{}{} pre-push \"$@\"",
            env,
            git_lfs.display()
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
        self.run("lfs", &[OsStr::new("fetch")])?;
        Ok(())
    }


    /// Looks at .gitattributes and committed/staged pointer files
    /// WARNING: Returned paths are lossy UTF-8
    pub fn lfs_ls_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let output = self.run("lfs", &[
            OsStr::new("ls-files"),
            OsStr::new("--name-only")
        ])?;

        let files =
            Self::lossy_and_trim(&output.stdout)
            .lines()
            .map(PathBuf::from)
            .collect();

        Ok(files)
    }


    /// Puts the path in .gitattributes under LFS filters
    pub fn lfs_track(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if path.file_name() == Some(OsStr::new(".gitattributes")) {
            return Err("Cannot track .gitattributes".into());
        }

        self.run("lfs", &[
            OsStr::new("track"),
            OsStr::new("--"), // Safety: No more flags coming after this
            path.as_os_str(),
        ])?;

        Ok(())
    }


    pub fn lfs_version(&self) -> Option<String> {
        self.run("lfs", &[OsStr::new("--version")])
            .ok()
            .map(|o| Self::lossy_and_trim(&o.stdout))
    }
}
