//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::git::objects::reference::GitReference;

use super::objects::environment::GitEnvironment;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-merge

    pub fn merge(&self, ref_str: &GitReference) -> Result<(), Box<dyn Error>> {
        if self.is_in_merge() {
            // Note: Never use `git-merge --abort` as it can cause data loss
            return Err("Already in a merge".into());
        }

        let output = self.run("merge", &[
            OsStr::new("-S"), // Sign the merge commit (not done implicitly on merge) // TODO: test with long flag --gpg-sign
            OsStr::new("--no-edit"), // Don't get blocked by interactive editors
            OsStr::new(ref_str),
        ])?;

        if output.status.success() {
            Ok(())
        } else {
            Err("Merge failed".into())
        }
    }


    pub fn merge_blame(&self, path: &Path) -> Result<GitUser, Box<dyn Error>> {
        if !self.is_in_merge() {
            return Err("Not in a merge".into());
        }

        let output = self.run("log", &[
            OsStr::new("--format=%an <%ae>"),
            OsStr::new("--max-count=1"),
            OsStr::new("FETCH_HEAD"),
            OsStr::new("--"),
            path.as_os_str(),
        ])?;

        String::from_utf8_lossy(&output.stdout)
            .parse::<GitUser>()
    }


    pub fn is_in_merge(&self) -> bool {
        self.working_dir
            .join(".git")
            .join("MERGE_HEAD")
            .exists()
    }
}
