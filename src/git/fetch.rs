//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-fetch

    pub fn fetch(&self, branch: &str) -> Result<bool, Box<dyn Error>> {
        let output = self.run("fetch", &[
            "--no-recurse-submodules",
            "--progress", // Print progress on stderr
            "origin",
            branch
        ])?;

        match output.exit_code {
            0   => {}, // Fetch completed successfully
            1   => {},
            2   => return Err("Error: ...".into()),
            128 => return Err("Error: No connection".into()),
            _   => return Err("Error: Unknown error".into()),
        }

        Ok(true)
    }
}
