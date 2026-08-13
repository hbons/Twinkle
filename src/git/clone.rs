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

    pub fn clone(&self, url: &str, directory: Option<&Path>, depth: Option<u32>) -> Result<GitEnvironment, Box<dyn Error>> {
        let mut args: Vec<&OsStr> = Vec::new();

        let mut depth_str = "--depth=".to_string();
        if let Some(d) = depth {
            depth_str.push_str(&format!("{}", d));
            args.push(OsStr::new(&depth_str));
        }

        args.push(OsStr::new("--no-checkout"));
        args.push(OsStr::new("--progress"));
        args.push(OsStr::new("--")); // Safety: No more flags coming after this
        args.push(OsStr::new(url));

        if let Some(dir) = directory {
            args.push(dir.as_os_str());
        }

        self.run("clone", &args)?;

        let url = url.parse::<SshUrl>()?;

        let dir_name = match directory {
            Some(d) => d.file_name().ok_or("Could not get name from path")?,
            None => url.path.file_name().ok_or("Could not get name from url")?,
        };

        let mut git_env = Clone::clone(self);
        git_env.working_dir = self.working_dir.join(dir_name);

        Ok(git_env)
    }
}
