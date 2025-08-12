//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::process::Command;

use crate::log;

use super::keygen::ssh_keygen_fingerprint;
use super::keys::host_key::HostKey;
use super::keys::key_type::KeyType;


pub fn ssh_keyscan(host: &str, port: Option<u16>, key_type: KeyType) -> Result<HostKey, Box<dyn Error>> {
    let port = port.unwrap_or(22);

    let args = [
        "-q", // Don't print server host name and banners in comments
        "-t", &key_type.to_string(), // Key type
        &format!("-p {port}"), // Port
        host, // Host
    ];

    log::debug(&format!("ssh-keyscan {}", &args.join(" ")));

    let mut command = Command::new("ssh-keyscan");

    if env::var("GITHUB_ACTIONS").is_err() {
        command.env_clear(); // Causes GitHub Actions to not find the compiled OpenSSH
    }

    let result = command.args(args).output();

    match result {
        Ok(output) => {
            if !output.status.success() {
                log::error(String::from_utf8_lossy(&output.stderr).trim());
                let code = output.status.code().unwrap_or_default();
                return Err(format!("ssh-keyscan exited with error {code}").into());
            }

            // log::log_info(&String::from_utf8_lossy(&output.stdout).trim());

            let line = String::from_utf8_lossy(&output.stdout);
            let public_key = line.split_whitespace().nth(2).ok_or("No key part")?.to_string();

            let mut host_key = HostKey {
                host: host.to_string(),
                is_trusted: false,
                key_type,
                public_key,
                fingerprint: None,
            };

            if let Ok(fingerprint) = ssh_keygen_fingerprint(&host_key) {
                host_key.fingerprint = Some(fingerprint);
            }

            Ok(host_key)
        },
        Err(e) => Err(format!("Could not run ssh-keyscan: {e}").into())
    }
}
