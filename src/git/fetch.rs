//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;

use super::objects::environment::GitEnvironment;
use super::objects::reference::GitReference;
use super::objects::remote::GitRemote;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-fetch

    pub fn fetch(&self,
        remote: &GitRemote,
        branch: &GitReference,
    ) -> Result<(), Box<dyn Error>>
    {
        let output = self.run("fetch", &[
            OsStr::new("--no-recurse-submodules"),
            OsStr::new("--progress"), // Print progress on stderr
            OsStr::new(remote),
            OsStr::new(branch),
        ])?;

        match output.status.code() {
            Some(0)   => (), // Fetch completed successfully
            Some(1)   => (),
            Some(2)   => return Err("Error: ...".into()),
            Some(128) => return Err("Error: No connection".into()),
            Some(c)   => return Err(format!("Error: {c}").into()),
            None      => return Err("Error: Unknown error".into()),
        }

        Ok(())
    }
}
