//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::fs;

use super::objects::environment::GitEnvironment;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-commit

    pub fn commit(&self, author: Option<GitUser>, message: &str) -> Result<(), Box<dyn Error>> {
        let path = self.working_dir
            .join(".git")
            .join("COMMIT_EDITMSG");

        let args = &[
            OsStr::new("--no-edit"),
            OsStr::new("--file"),
            path.as_os_str(),
        ];

        let env = author.map(|user| vec![
            ("GIT_AUTHOR_NAME".into(), user.name().into()),
            ("GIT_AUTHOR_EMAIL".into(), user.email().into()),
            ("GIT_COMMITTER_NAME".into(), user.name().into()),
            ("GIT_COMMITTER_EMAIL".into(), user.email().into()),
        ]).unwrap_or_default();

        fs::write(&path, message)?; // Use a file to prevent encoding problems
        self.run_with_env("commit", args, &env)?;
        fs::remove_file(path)?;

        Ok(())
    }
}
