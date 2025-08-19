//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;

use super::objects::environment::GitEnvironment;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-commit

    pub fn commit(&self, author: &GitUser, message: &str) -> Result<(), Box<dyn Error>> {
        let env: Vec<(String, String)> = vec![
            ("GIT_AUTHOR_NAME".into(), author.name().into()),
            ("GIT_AUTHOR_EMAIL".into(), author.email().into()),
            ("GIT_COMMITTER_NAME".into(), author.name().into()),
            ("GIT_COMMITTER_EMAIL".into(), author.email().into()),
        ];

        let path = ".git/COMMIT_EDITMSG".to_string();
        let abs_path = self.working_dir.join(&path);
        fs::write(&abs_path, message)?; // Use a file to prevent encoding problems

        let file_arg = &format!("--file={}", path);
        self.run_with_env("commit", &[file_arg, "--no-edit"], env)?;
        fs::remove_file(abs_path)?;

        Ok(())
    }
}
