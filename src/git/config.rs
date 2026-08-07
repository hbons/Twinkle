//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use super::objects::environment::GitEnvironment;
use super::objects::output::GitOutput;


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

    pub fn config_get(&self, name: &str) -> Option<GitOutput> {
        self.run("config", &["--local", name]).ok()
    }

    pub fn config_set(&self, name: &str, value: &str)
    -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &["--local", name, value])
    }


    pub fn config_file_get(
        &self,
        file: &Path,
        name: &str,
    ) -> Option<GitOutput>
    {
        let file = file.to_string_lossy().to_string();
        self.run("config", &["--file", &file, name]).ok()
    }

    pub fn config_file_set(&self,
        file: &Path,
        name: &str,
        value: &str,
    ) -> Result<GitOutput, Box<dyn Error>>
    {
        let file = file.to_string_lossy().to_string();
        self.run("config", &["--file", &file, name, value])
    }


    pub fn config_list(&self) -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &["--local", "--list"])
    }
}
