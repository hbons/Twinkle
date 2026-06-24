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

    pub fn commit(&self, author: Option<GitUser>, message: &str) -> Result<(), Box<dyn Error>> {
        let path = ".git/COMMIT_EDITMSG".to_string();
        let abs_path = self.working_dir.join(&path);
        fs::write(&abs_path, message)?; // Use a file to prevent encoding problems

        let args = &[
            &format!("--file={}", path),
            "--no-edit",
        ];

        match author {
            Some(user) =>{
                let env: Vec<(String, String)> = vec![
                    ("GIT_AUTHOR_NAME".into(), user.name().into()),
                    ("GIT_AUTHOR_EMAIL".into(), user.email().into()),
                    ("GIT_COMMITTER_NAME".into(), user.name().into()),
                    ("GIT_COMMITTER_EMAIL".into(), user.email().into()),
                ];

                self.run_with_env("commit", args, env)
            },
            None => self.run("commit", args),
        }?;

        fs::remove_file(abs_path)?;

        Ok(())
    }
}
