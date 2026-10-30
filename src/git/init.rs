//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::ffi::OsStr;

use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::reference::GitReference;


// Still "master" until Git 3.0
const DEFAULT_BRANCH: &str = "main";

impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-init

    pub fn init(&self) -> Result<GitReference, Box<dyn Error>> {
        self.run("init", &[
            OsStr::new(&format!("--initial-branch={DEFAULT_BRANCH}")),
            OsStr::new("--quiet"),
        ])?;

        Ok(DEFAULT_BRANCH.into())
    }
}
