//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::ssh::objects::url::SshUrl;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-clone

    pub fn clone(&self, url: &str, directory: Option<&Path>) -> Result<GitEnvironment, Box<dyn Error>> {
        let mut args = vec![
            OsStr::new("--no-checkout"),
            OsStr::new("--progress"),
            OsStr::new("--"), // Safety: No more flags coming after this
            OsStr::new(url),
        ];

        if let Some(dir) = directory {
            args.push(dir.as_os_str()); // TODO: unwrap_or_default?
        }

        self.run("clone", &args)?;

        let url = url.parse::<SshUrl>()?;

        let dir_name = match directory {
            Some(d) => d.file_name().ok_or("Could not get name from path")?,
            None => url.path.file_name().ok_or("Could not get name from url")?,
        };

        let mut git = Clone::clone(self);
        git.working_dir = self.working_dir.join(dir_name); // TODO: Allow absolute paths?

        Ok(git)
    }
}
