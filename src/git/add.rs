//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-add

    pub fn add(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let path = path.to_str().ok_or("Path is not valid UTF-8")?;
        self.run("add", &[
            "--", // Safety: no more flags coming after this
            path,
        ])?;

        Ok(())
    }
}
