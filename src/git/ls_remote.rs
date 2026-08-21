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

    pub fn ls_remote(&self, remote: &GitRemote, branch: &GitReference) -> Result<String, Box<dyn Error>> {
        let output = self.run("ls-remote", &[
            OsStr::new("--exit-code"), // Use exit codes on errors
            OsStr::new("--heads"), // '--branches' after Git 2.46.0 (Sep 11 2024)
            OsStr::new("--quiet"), // Don't print remote to stderr
            OsStr::new("--"), // Safety: No more flags coming after this
            OsStr::new(remote),
            OsStr::new(branch),
        ])?;

        match output.status.code() {
            Some(0)   => (), // Successful connection
            Some(2)   => return Err("No matching remote branch".into()),
            Some(128) => return Err("No connection".into()),
            Some(c)   => return Err(format!("Unknown error: {c}").into()),
            None      => return Err("Unknown error".into()),
        }

        let output = String::from_utf8_lossy(&output.stdout)
            .to_string(); // TODO: needed?

        // '950264636c68591989456e3ba0a5442f93152c1a	refs/heads/main'
        output.split('\t').next()
            .map(|remote_id| remote_id.to_string())
            .ok_or_else(|| "Cannot parse remote id".into())
    }
}
