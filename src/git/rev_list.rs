//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-rev-list

    pub fn rev_list_count(&self) -> Result<u32, Box<dyn Error>> {
        let output = self.run("rev-list", &[
            OsStr::new("--count"),
            OsStr::new("@{u}..HEAD")
        ])?;

        Ok(
            Self::lossy_and_trim(&output.stdout)
                .parse::<u32>()?
        )
    }
}
