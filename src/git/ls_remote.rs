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
    // Docs: https://git-scm.com/docs/git-ls-remote

    pub fn ls_remote(
        &self,
        remote: &GitRemote,
        branch: &GitReference,
    ) -> Result<GitReference, Box<dyn Error>>
    {
        let output = self.run("ls-remote", &[
            OsStr::new("--exit-code"), // Use exit codes on errors
            OsStr::new("--quiet"), // Don't print remote to stderr
            OsStr::new("--"), // Safety: No more flags coming after this
            OsStr::new(remote),
            OsStr::new(branch),
        ])?;

        match output.status.code() {
            Some(0)    => (), // Successful connection
            Some(2)    => return Err("No matching remote branch".into()),
            Some(128)  => return Err("No connection".into()),
            Some(code) => return Err(format!("Unknown error: {code}").into()),
            None       => return Err("Unknown error".into()),
        }

        // '950264636c68591989456e3ba0a5442f93152c1a\trefs/heads/main'
        Self::lossy_and_trim(&output.stdout)
            .split('\t')
            .next()
            .map(|id| id.into())
            .ok_or_else(|| "Could not parse remote id".into())
    }
}
