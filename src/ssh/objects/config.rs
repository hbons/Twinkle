//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::path::PathBuf;
use std::time::Duration;


#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct SshConfig {
    // Docs: https://man.openbsd.org/ssh_config

    pub BatchMode: bool,
    pub ConnectionAttempts: u32,
    pub ConnectTimeout: Duration,
    pub IdentitiesOnly: bool,
    pub IdentityFile: PathBuf,
    pub PasswordAuthentication: bool,
    pub ServerAliveCountMax: u32,
    pub ServerAliveInterval: Duration,
    pub StrictHostKeyChecking: bool,
    pub UserKnownHostsFile: PathBuf,
}


impl Default for SshConfig {
    fn default() -> SshConfig {
        // Note: Can't use "UpdateHostKeys=yes" as it fetches all
        //       key types which messes with fingerprinting

        SshConfig {
            BatchMode: true,
            ConnectionAttempts: 2,
            ConnectTimeout: Duration::from_secs(4),
            IdentitiesOnly: true,
            IdentityFile: PathBuf::new(),
            PasswordAuthentication: false,
            ServerAliveCountMax: 2,
            ServerAliveInterval: Duration::from_secs(4),
            StrictHostKeyChecking: true,
            UserKnownHostsFile: PathBuf::new(),
        }
    }
}


impl fmt::Display for SshConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options = vec![
            format!("-o BatchMode={}", format_bool(self.BatchMode)),
            format!("-o ConnectionAttempts={}", self.ConnectionAttempts),
            format!("-o ConnectTimeout={}", self.ConnectTimeout.as_secs()),
            format!("-o IdentitiesOnly={}", format_bool(self.IdentitiesOnly)),
            format!("-o IdentityFile={}", self.IdentityFile.to_string_lossy()),
            format!("-o PasswordAuthentication={}", format_bool(self.PasswordAuthentication)),
            format!("-o ServerAliveCountMax={}", self.ServerAliveCountMax),
            format!("-o ServerAliveInterval={}", self.ServerAliveInterval.as_secs()),
            format!("-o StrictHostKeyChecking={}", format_bool(self.StrictHostKeyChecking)),
            format!("-o UserKnownHostsFile={}", self.UserKnownHostsFile.to_string_lossy()),
        ];

        write!(f, "{}", options.join(" "))
    }
}


fn format_bool(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
