//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-push

    pub fn push(&self, remote_url: &str, branch: &str) -> Result<(), Box<dyn Error>> {
        self.run("push", &[
            "--progress",
            remote_url,
            branch,
        ])?;

        Ok(())
    }
}
