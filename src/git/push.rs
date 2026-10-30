//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::ffi::OsStr;

use super::objects::environment::GitEnvironment;
use super::objects::reference::GitReference;
use super::objects::remote::GitRemote;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-push

    pub fn push(&self,
        remote: &GitRemote,
        branch: &GitReference,
    ) -> Result<(), Box<dyn Error>>
    {
        self.run("push", &[
            OsStr::new("--progress"),
            OsStr::new("--set-upstream"),
            OsStr::new(remote),
            OsStr::new(branch),
        ])?;

        Ok(())
    }
}
