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

    pub fn clone(
        &self,
        url: &SshUrl,
        target_dir: &Path,
    ) -> Result<GitEnvironment, Box<dyn Error>>
    {
        let url_str = url.to_string_standard();

        let args = vec![
            OsStr::new("--no-checkout"),
            OsStr::new("--progress"),
            OsStr::new("--"), // Safety: No more flags coming after this
            OsStr::new(&url_str),
            OsStr::new(&target_dir),
        ];

        self.run("clone", &args)?;
        let mut git = Clone::clone(self);

        // TODO: we need to add a integration test cloning to relative and absolute path arg

        git.working_dir = {
            if target_dir.is_absolute() {
                target_dir.to_path_buf()
            } else {
                let dir_name = target_dir
                    .file_name()
                    .ok_or("Could not get name from path")?;

                self.working_dir.join(dir_name)
            }
        };

        Ok(git)
    }
}
