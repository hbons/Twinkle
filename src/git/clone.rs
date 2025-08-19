//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use crate::ssh::objects::url::SshUrl;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-clone

    pub fn clone(&self, url: &str, directory: Option<&Path>, depth: Option<u32>) -> Result<GitEnvironment, Box<dyn Error>> {
        let mut args: Vec<&str> = Vec::new();

        let mut depth_str = "--depth=".to_string();
        if let Some(d) = depth {
            depth_str.push_str(&format!("{}", d));
            args.push(&depth_str);
        }

        args.push("--no-checkout");
        args.push("--progress");
        args.push(url);

        if let Some(d) = directory {
            let dir_str = d.to_str().ok_or("Invalid directory path")?;
            args.push(dir_str);
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
