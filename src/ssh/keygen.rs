//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::error::Error;
use std::fs::{ create_dir_all, read_to_string };
use std::io::Write;
use std::path::Path;
use std::process::{ Command, Stdio };

use crate::log;

use super::keys::fingerprint::Fingerprint;
use super::keys::host_key::HostKey;
use super::keys::key_pair::KeyPair;
use super::keys::key_size::KeySize;
use super::keys::key_type::KeyType;


pub fn ssh_keygen(key_path: &Path, key_type: KeyType, key_size: Option<KeySize>) -> Result<KeyPair, Box<dyn Error>> {
    // Docs: https://man.openbsd.org/ssh-keygen

    let keys_dir = key_path.parent().ok_or("Could not find parent directory")?;

    if !keys_dir.exists() {
        create_dir_all(keys_dir)?;
    }

    let key_size = match key_size {
        Some(key_size) => key_size,
        None => KeySize::default(key_type),
    };

    let args = [
        // "-q", // Quiet
        "-t", &key_type.to_string(), // Key type
        "-b", &key_size.to_string(), // Key size in bits
        "-P", "", // No passphrase
        "-C", "Twinkle", // Key comment
        "-f", key_path.to_str().ok_or("Invalid key path")?, // File name
    ];

    log::debug(&format!("ssh-keygen {}", &args.join(" ")));

    let mut command = Command::new("ssh-keygen");

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

            let pubkey_path = key_path.with_extension("key.pub");

            let key_pair = KeyPair {
                key_type,
                private_key: read_to_string(key_path)?.trim().to_string(),
                private_key_path: key_path.to_path_buf(),
                passphrase: None,

                public_key:  read_to_string(&pubkey_path)?.trim().to_string(),
                public_key_path: pubkey_path.to_path_buf(),
            };

            Ok(key_pair)
        },
        Err(e) => Err(format!("Could not run ssh-keygen: {e}").into())
    }
}


pub fn ssh_keygen_fingerprint(host_key: &HostKey) -> Result<Fingerprint, Box<dyn Error>> {
    // Docs: https://man.openbsd.org/ssh-keygen#l

    let mut child = Command::new("ssh-keygen")
        .env_clear()
        .arg("-q") // Quiet
        .arg("-l") // Display fingerprint
        .arg("-E").arg("sha256") // Hash algorithm
        .arg("-f").arg("/dev/stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Could not open stdin")?;
        stdin.write_all(host_key.to_string().as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        let line = String::from_utf8_lossy(&output.stdout);
        let line = line.split_whitespace().nth(1).ok_or("Missing fingerprint output")?;

        Ok(line.parse::<Fingerprint>()?)
    } else {
        Err(format!("Could not derive fingerprint: {}",
            String::from_utf8_lossy(&output.stderr)).into())
    }
}
