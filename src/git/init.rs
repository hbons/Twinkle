//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::reference::GitReference;


const DEFAULT_BRANCH: &str = "main"; // Still "master" until Git 3.0

impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-init

    pub fn init(&self) -> Result<GitReference, Box<dyn Error>> {
        self.run("init", &[
            &format!("--initial-branch={DEFAULT_BRANCH}"),
            "--quiet",
        ])?;

        Ok(DEFAULT_BRANCH.into())
    }
}
