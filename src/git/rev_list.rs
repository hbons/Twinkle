//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-rev-list

    pub fn rev_list_count(&self) -> Result<u32, Box<dyn Error>> {
        let output = self.run("rev-list", &["--count", "@{u}..HEAD"])?;
        let count = output.stdout.parse::<u32>()?;

        Ok(count)
    }
}
