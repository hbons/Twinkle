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
use super::objects::version::GitVersion;


pub const K_CORE_IGNORE_CASE: &str = "core.ignoreCase";
pub const K_CORE_SSH_COMMAND: &str = "core.sshCommand";

pub const K_REMOTE_ORIGIN_URL: &str = "remote.origin.url";

pub const K_USER_NAME: &str = "user.name";
pub const K_USER_EMAIL: &str = "user.email";
pub const K_USER_SIGNING_KEY: &str = "user.signingKey";

pub const K_COMMIT_GPG_SIGN: &str = "commit.gpgSign";
pub const K_TAG_GPG_SIGN: &str = "tag.gpgSign";


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-config
    //       https://git-scm.com/docs/git-config#_deprecated_modes

    pub fn config_get(
        &self,
        name: &str,
    ) -> Option<String>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => vec![
                OsStr::new("--local"),
                OsStr::new("get"),
                OsStr::new(name),
            ],
            _ => vec![
                OsStr::new("--local"),
                OsStr::new(name),
            ],
        };

        self.run("config", &args).ok()
            .map(|o| Self::lossy_and_trim(&o.stdout))
    }


    pub fn config_set(
        &self,
        name: &str,
        value: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => vec![
                OsStr::new("--local"),
                OsStr::new("set"),
                OsStr::new(name),
                OsStr::new(value),
            ],
            _ => vec![
                OsStr::new("--local"),
                OsStr::new(name),
                OsStr::new(value),
            ],
        };

        self.run("config", &args).map(drop)
    }


    pub fn config_unset(
        &self,
        name: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => &[
                OsStr::new("--local"),
                OsStr::new("unset"),
                OsStr::new(name),
            ],
            _ => &[
                OsStr::new("--local"),
                OsStr::new("--unset"),
                OsStr::new(name),
            ],
        };

        self.run("config", args).ok()
            .map(|o| Self::lossy_and_trim(&o.stdout))
    }
}


impl GitEnvironment {
    pub fn config_get_with_file(
        &self,
        name: &str,
        path: &Path,
    ) -> Option<String>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => vec![
                OsStr::new("--file"),
                path.as_os_str(),
                OsStr::new("get"),
                OsStr::new(name),
            ],
            _ => vec![
                OsStr::new("--file"),
                path.as_os_str(),
                OsStr::new(name),
            ],
        };

        self.run("config", &args).ok()
            .map(|o| Self::lossy_and_trim(&o.stdout))
    }


    pub fn config_set_with_file(&self,
        name: &str,
        value: &str,
        file_path: &Path,
    ) -> Result<(), Box<dyn Error>>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => vec![
                OsStr::new("--file"),
                file_path.as_os_str(),
                OsStr::new("set"),
                OsStr::new(name),
                OsStr::new(value),
            ],
            _ => vec![
                OsStr::new("--file"),
                file_path.as_os_str(),
                OsStr::new(name),
                OsStr::new(value),
            ],
        };

        self.run("config", &args).map(drop)
    }


    pub fn config_unset_with_file(
        &self,
        name: &str,
        path: &Path,
    ) -> Result<(), Box<dyn Error>>
    {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => &[
                OsStr::new("--file"),
                path.as_os_str(),
                OsStr::new("unset"),
                OsStr::new(name),
            ],
            _ => &[
                OsStr::new("--file"),
                path.as_os_str(),
                OsStr::new("--unset"),
                OsStr::new(name),
            ],
        };

        self.run("config", args).map(drop)
    }
}


impl GitEnvironment {
    pub fn config_list(&self) -> Result<Output, Box<dyn Error>> {
        let args = match self.version {
            Some(GitVersion::Git3(_)) => &[
                OsStr::new("--local"),
                OsStr::new("list"),
            ],
            _ => &[
                OsStr::new("--local"),
                OsStr::new("--list"),
            ],
        };

        self.run("config", args)
    }
}
