//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-checkout

    pub fn checkout_branch(&self, branch: &str) -> Result<(), Box<dyn Error>> {
        self.run("checkout", &[
            "--quiet",
            branch])?;

        Ok(())
    }


    pub fn checkout_file(&self, path: &Path, extra_arg: Option<&str>) -> Result<(), Box<dyn Error>> {
        self.run("checkout", &[
            extra_arg.unwrap_or(""),
            path.to_str().ok_or("Path is not valid UTF-8")?,
        ])?;

        Ok(())
    }

    pub fn checkout_ours(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.checkout_file(path, Some("--ours"))?;
        Ok(())
    }

    pub fn checkout_theirs(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.checkout_file(path, Some("--theirs"))?;
        Ok(())
    }

    pub fn checkout_original(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.checkout_file(path, Some("ORIG_HEAD^"))?;
        Ok(())
    }
}
