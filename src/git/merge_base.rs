//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-merge-base

    pub fn merge_base(&self, commit_id: &str, branch: &str) -> Result<bool, Box<dyn Error>> {
        let result = self.run("merge-base", &[
            "--is-ancestor",
            commit_id,
            branch,
        ]);

        match result {
            Ok(output) if output.exit_code == 0 => Ok(true), // Commit is in the branch's history
            Ok(output) if output.exit_code == 1 => Ok(false), // It's not
            Ok(_) => Ok(false),  // It's not (commit is probably remote)
            Err(_) => Ok(false), // It's not (commit is probably remote)
        }
    }
}
