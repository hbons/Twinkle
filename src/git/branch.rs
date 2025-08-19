//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-branch

        pub fn branch_show_current(&self) -> Result<String, Box<dyn Error>> {
            let output = self.run("branch", &["--show-current"])?;
            let branch = output.stdout;

            match branch.as_str() {
                "" => Err("No branch found".into()),
                _  => Ok(branch),
            }
        }
}
