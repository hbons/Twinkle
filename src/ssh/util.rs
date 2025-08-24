//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::process::Command;

use crate::log;

use super::keys::host_key::HostKey;
use super::keys::key_pair::KeyPair;
use super::objects::config::SshConfig;
use super::objects::url::SshUrl;


pub fn ssh_util_test_connection(url: &SshUrl, host_key: &HostKey, key_pair: &KeyPair) -> Result<(), Box<dyn Error>> {
    let hostkey_name = host_key.to_file_name();
    let hostkey_path = key_pair.private_key_path.parent().ok_or("No parent directory")?;
    let hostkey_path = hostkey_path.join(hostkey_name);

    let config = SshConfig {
        IdentityFile: key_pair.private_key_path.clone(),
        UserKnownHostsFile: hostkey_path,
        ..Default::default()
    };

    let args = format!("-T {} {}@{}", config, url.user, url.host);
    log::debug(&format!("ssh {}", args));
    let args: Vec<&str> = args.split_whitespace().collect();

    let ssh = Command::new("ssh")
        .args(args)
        .output()?;

    log::debug(format!("{}", String::from_utf8_lossy(&ssh.stderr)).trim());

    match ssh.status.code() {
        Some(0) => Ok(()),
        Some(1) if url.host == "github.com" => Ok(()), // GitHub is special
        Some(n) => Err(format!("ssh exited with code {n}").into()),
        None => Err("Could not run ssh".into()),
    }
}
