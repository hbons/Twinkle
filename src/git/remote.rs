//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;

use crate::ssh::objects::url::SshUrl;

use super::objects::environment::GitEnvironment;
use super::objects::remote::GitRemote;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-push

    pub fn remote_add(&self,
        remote: &GitRemote,
        url: &SshUrl,
    ) -> Result<(), Box<dyn Error>>
    {
        self.run("remote", &[
            OsStr::new("add"),
            OsStr::new("--"),
            OsStr::new(remote),
            OsStr::new(&url.to_string_standard()),
        ]).map(drop)
    }
}
