//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::process::Command;

use crate::log;

use super::keys::host_key::HostKey;
use super::keys::key_pair::KeyPair;
use super::objects::url::SshUrl;


pub fn ssh_util_test_connection(url: &SshUrl, host_key: &HostKey, key_pair: &KeyPair) -> Result<(), Box<dyn Error>> {
    let hostkey_name = host_key.to_file_name();
    let hostkey_path = key_pair.private_key_path.parent().ok_or("No parent directory")?;
    let hostkey_path = hostkey_path.join(hostkey_name);
    let hostkey_path = hostkey_path.to_str().ok_or("Invalid host key path")?;

    let private_key_path = key_pair.private_key_path.to_string_lossy();

    let args = [
        "-T", // Test the connection
        "-o BatchMode=yes", // Don't block on any prompts
        "-o ConnectTimeout=16",
        "-o IdentitiesOnly=yes",
        "-o", &format!("IdentityFile={}", private_key_path),
        "-o PasswordAuthentication=no", // Don't block on host password prompts
        "-o StrictHostKeyChecking=yes",
        "-o", &format!("UserKnownHostsFile={}", hostkey_path),
        &format!("{}@{}", &url.user, &url.host)
    ];

    log::debug(&format!("ssh {}", &args.join(" ")));

    let mut command = Command::new("ssh");

    if env::var("GITHUB_ACTIONS").is_err() {
        command.env_clear(); // Causes GitHub Actions to not find the compiled OpenSSH
    }

    let output = command.args(args).output()?;
    log::debug(format!("{}", String::from_utf8_lossy(&output.stderr)).trim());

    match output.status.code() {
        Some(0) => Ok(()),
        Some(1) if url.host == "github.com" => Ok(()), // GitHub is special
        Some(n) => Err(format!("ssh exited with code {n}").into()),
        None => Err("Could not run ssh".into()),
    }
}
