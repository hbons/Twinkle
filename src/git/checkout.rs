//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use super::objects::environment::GitEnvironment;
use super::objects::reference::GitReference;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-checkout

    pub fn checkout_branch(&self, branch: &GitReference) -> Result<(), Box<dyn Error>> {
        self.run("checkout", &[
            OsStr::new("--quiet"),
            OsStr::new(branch),
        ])?;

        Ok(())
    }


    pub fn checkout_file(&self, path: &Path, extra_arg: Option<&OsStr>) -> Result<(), Box<dyn Error>> {
        self.run("checkout", &[
            extra_arg.unwrap_or_default(),
            OsStr::new("--"), // Safety: No more flags coming after this
            path.as_os_str()
        ])?;

        Ok(())
    }


    pub fn checkout_ours(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.checkout_file(path, Some(OsStr::new("--ours"))) // same as checkout-index --stage=2
    }

    pub fn checkout_theirs(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.checkout_file(path, Some(OsStr::new("--theirs"))) // same as checkout-index --stage=3
    }

    pub fn checkout_common_ancestor(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.run("checkout-index", &[
            OsStr::new("--stage=1"), // Common ancestor
            OsStr::new("--"),
            path.as_os_str(),
        ])?;

        Ok(())
    }
}
