//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::path::PathBuf;


#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct SshConfig {
    // Docs: https://man.openbsd.org/ssh_config

    pub BatchMode: bool,
    pub IdentitiesOnly: bool,
    pub IdentityFile: PathBuf,
    pub PasswordAuthentication: bool,
    pub StrictHostKeyChecking: bool,
    pub UserKnownHostsFile: PathBuf,
}


impl Default for SshConfig {
    fn default() -> SshConfig {
        // Note: Can't use "UpdateHostKeys=yes" as it fetches all
        //       key types which messes with fingerprinting

        SshConfig {
            BatchMode: true,
            IdentitiesOnly: true,
            IdentityFile: PathBuf::new(),
            PasswordAuthentication: false,
            StrictHostKeyChecking: true,
            UserKnownHostsFile: PathBuf::new(),
        }
    }
}


impl fmt::Display for SshConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options = vec![
            format!("-o BatchMode={}", format_bool(self.BatchMode)),
            format!("-o IdentitiesOnly={}", format_bool(self.IdentitiesOnly)),
            format!("-o IdentityFile={}", self.IdentityFile.to_string_lossy()),
            format!("-o PasswordAuthentication={}", format_bool(self.PasswordAuthentication)),
            format!("-o StrictHostKeyChecking={}", format_bool(self.StrictHostKeyChecking)),
            format!("-o UserKnownHostsFile={}", self.UserKnownHostsFile.to_string_lossy()),
        ];

        write!(f, "{}", options.join(" "))
    }
}


fn format_bool(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
