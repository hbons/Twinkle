//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use super::objects::environment::GitEnvironment;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-merge

    pub fn merge(&self, ref_str: &str) -> Result<(), Box<dyn Error>> {
        if self.is_in_merge() {
            // Note: Never use `git-merge --abort` as it can cause data loss
            return Err("Already in a merge".into());
        }

        let output = self.run("merge", &[
            "-S", // Sign the merge commit (not done implicitly on merge)
            "--no-edit", // Don't get blocked by interactive editors
            ref_str
        ])?;

        match output.exit_code {
            0 => Ok(()),
            _ => Err("Merge failed".into()),
        }
    }


    pub fn merge_blame(&self, path: &Path) -> Result<GitUser, Box<dyn Error>> {
        if !self.is_in_merge() {
            return Err("Not in a merge".into());
        }

        let output = self.run("log", &[
            "--format=%an <%ae>",
            "--max-count=1",
            "FETCH_HEAD",
            "--",
            path.to_str().ok_or("Path is not valid UTF-8")?
        ])?;

        Ok(output.stdout.parse::<GitUser>()?)
    }


    pub fn is_in_merge(&self) -> bool {
        self.working_dir.join(".git").join("MERGE_HEAD").exists()
    }
}
