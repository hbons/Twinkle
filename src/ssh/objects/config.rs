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

    pub BatchMode: bool, // password prompts and host key confirmation requests will be disabled
    pub ConnectionAttempts: u32, // number of tries (one per second) to make before exiting. The argument must be an integer. This may be useful in scripts if the connection sometimes fails. The default is 1.
    pub ConnectTimeout: Duration, // instead of using the default system TCP timeout.
    pub IdentitiesOnly: bool, //  only use the configured authentication identity and certificate files (either the default files, or those explicitly configured in the ssh_config files or passed on the ssh(1) command-line), even if ssh-agent(1) or a PKCS11Provider or SecurityKeyProvider offers more identities
    pub IdentityFile: Option<PathBuf>, // Specifies a file from which the user's ECDSA, authenticator-hosted ECDSA, Ed25519, authenticator-hosted Ed25519 or RSA authentication identity is read
    pub PasswordAuthentication: bool, //  whether to use password authenticatio
    pub ServerAliveCountMax: u32, // number of ServerAliveInterval after which to disconnect
    pub ServerAliveInterval: Duration, // Sets a timeout interval in seconds after which if no data has been received from the server, ssh(1) will send a message through the encrypted channel to request a response from the server.
    pub StrictHostKeyChecking: bool, //  never automatically add host keys to the ~/.ssh/known_hosts file, and refuses to connect to hosts whose host key has changed
    pub UserKnownHostsFile: Option<PathBuf>, //  file to use for the user host key database,
}


impl Default for SshConfig {
    fn default() -> SshConfig {
        // Note: Can't use "UpdateHostKeys=yes" as it fetches all
        //       key types which messes with fingerprinting

        SshConfig {
            BatchMode: true,
            ConnectionAttempts: 2,
            ConnectTimeout: Duration::from_secs(4),
            IdentitiesOnly: false,
            IdentityFile: None,
            PasswordAuthentication: false,
            ServerAliveCountMax: 2,
            ServerAliveInterval: Duration::from_secs(4),
            StrictHostKeyChecking: true,
            UserKnownHostsFile: None,
        }
    }
}


impl fmt::Display for SshConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut options: Vec<String> = vec![
            format!("-o BatchMode={}", format_bool(self.BatchMode)),
            format!("-o ConnectionAttempts={}", self.ConnectionAttempts),
            format!("-o ConnectTimeout={}", self.ConnectTimeout.as_secs()),
            format!("-o IdentitiesOnly={}", format_bool(self.IdentitiesOnly)),
            format!("-o PasswordAuthentication={}", format_bool(self.PasswordAuthentication)),
            format!("-o ServerAliveCountMax={}", self.ServerAliveCountMax),
            format!("-o ServerAliveInterval={}", self.ServerAliveInterval.as_secs()),
            format!("-o StrictHostKeyChecking={}", format_bool(self.StrictHostKeyChecking)),
        ];

        if let Some(v) = &self.IdentityFile {
            options.push(
                format!("-o IdentityFile={}", v.to_string_lossy())
            );
        }

        if let Some(v) = &self.UserKnownHostsFile {
            options.push(
                format!("-o UserKnownHostsFile={}", v.to_string_lossy())
            );
        }

        options.sort();
        write!(f, "{}", options.join(" "))
    }
}


fn format_bool(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
