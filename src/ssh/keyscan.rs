//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::process::Command;

use crate::log;

use super::keygen;
use super::keys::host_key::HostKey;
use super::keys::key_type::KeyType;


/// Docs: https://man.openbsd.org/ssh-keyscan
pub fn scan_host(
    host: &str,
    port: Option<u16>,
    key_type: KeyType,
) -> Result<HostKey, Box<dyn Error>>
{
    let port = port.unwrap_or(22);

    let args = [
        "-q", // Don't print server host name and banners in comments
        "-t", &key_type.to_string(), // Key type
        &format!("-p {port}"), // Port
        host,
    ];

    log::debug(&format!("ssh-keyscan {}", args.join(" ")));

    let ssh_keyscan = Command::new("ssh-keyscan")
        .args(args)
        .output();

    match ssh_keyscan {
        Ok(output) => {
            if !output.status.success() {
                return Err(format!("ssh-keyscan exited with error {}: {}",
                    String::from_utf8_lossy(output.stderr.trim_ascii_end()),
                    output.status.code().unwrap_or_default(),
                ).into());
            }

            let line = String::from_utf8_lossy(
                output.stdout.trim_ascii_end()
            );

            let public_key = line
                .split_whitespace()
                .nth(2)
                .ok_or("No key part")?
                .to_string();

            let mut host_key = HostKey {
                host: host.to_string(),
                is_trusted: false,
                key_type,
                public_key,
                fingerprint: None,
            };

            if let Ok(fingerprint) = keygen::derive_fingerprint(&host_key) {
                host_key.fingerprint = Some(fingerprint);
            }

            Ok(host_key)
        },
        Err(e) => Err(format!("ssh-keyscan error: {e}").into())
    }
}
