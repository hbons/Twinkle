//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Output;

use super::objects::environment::GitEnvironment;


pub const K_CORE_SSH_COMMAND: &str = "core.sshCommand";
pub const K_REMOTE_ORIGIN_URL: &str = "remote.origin.url";

pub const K_USER_NAME: &str = "user.name";
pub const K_USER_EMAIL: &str = "user.email";
pub const K_USER_SIGNING_KEY: &str = "user.signingKey";

pub const K_COMMIT_GPG_SIGN: &str = "commit.gpgSign";
pub const K_TAG_GPG_SIGN: &str = "tag.gpgSign";


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-config

    // TODO: Implicit config operations are deprecated since Git 2.44.
    //       Use `git config get/set/list` if they exist.
    //       See: https://git-scm.com/docs/git-config#_deprecated_modes

    pub fn config_get(&self, name: &str) -> Option<Output> { // TODO: Option<String>
        self.run("config", &[
            OsStr::new("--local"),
            OsStr::new(name)
        ]).ok()
    }

    pub fn config_set(&self, name: &str, value: &str) -> Result<Output, Box<dyn Error>> { // TODO: Return () result
        self.run("config", &[
            OsStr::new("--local"),
            OsStr::new(name),
            OsStr::new(value),
        ])
    }


    pub fn config_file_get(
        &self,
        file_path: &Path,
        name: &str,
    ) -> Option<String>
    {
        self.run("config", &[
            OsStr::new("--file"),
            file_path.as_os_str(),
            OsStr::new(name),
        ]).map(|o|
            String::from_utf8_lossy(&o.stdout)
                .to_string()
        ).ok()
    }

    pub fn config_file_set(&self,
        file_path: &Path,
        name: &str,
        value: &str,
    ) -> Result<Output, Box<dyn Error>> // TODO: Return () result
    {
        self.run("config", &[
            OsStr::new("--file"),
            file_path.as_os_str(),
            OsStr::new(name),
            OsStr::new(value),
        ])
    }


    pub fn config_list(&self) -> Result<Output, Box<dyn Error>> {
        self.run("config", &[
            OsStr::new("--local"),
            OsStr::new("--list"),
        ])
    }
}
