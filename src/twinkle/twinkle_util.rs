//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::path::{ Path, PathBuf };

use crate::ssh::objects::config::SshConfig;
use crate::ssh::objects::url::SshUrl;
use crate::ssh::keys::key_pair::KeyPair;


// "ssh://git@github.com:hbons/Twinkle" -> "Twinkle"
// "ssh://git@github.com:hbons"         -> "hbons"
pub fn twinkle_default_dir_name(url: &SshUrl) -> Result<PathBuf, Box<dyn Error>> {
    let dir = url.path.file_stem().ok_or("Could not determine path")?;
    Ok(PathBuf::from(dir))
}


// "Projects/Folder" exists?   -> "Projects/Folder 2"
// "Projects/Folder 2" exists? -> "Projects/Folder 3" etc.
pub fn twinkle_unique_dir(dir: &Path) -> PathBuf {
    let mut unique_dir = dir.to_path_buf();
    let mut suffix = 2;

    while unique_dir.exists() {
        let path = format!("{} {suffix}", dir.display());
        unique_dir = Path::new(&path).to_path_buf();
        suffix += 1;
    }

    unique_dir.to_path_buf()
}


pub fn twinkle_ssh_command(key_pair: Option<&KeyPair>) -> String {
    let config = match key_pair {
        Some(key_pair) => {
            SshConfig {
                IdentitiesOnly: true,
                IdentityFile: Some(key_pair.private_key_path.clone()),
                UserKnownHostsFile: Some(key_pair.private_key_path.with_extension("key.host")),
                ..Default::default()
            }
        },
        None => SshConfig::default(),
    };

    format!("ssh -F /dev/null {config}")
}


/// Generates a random 256-bit (64 chars) hex string
pub fn twinkle_random_id() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("/dev/urandom")?;

    let mut buffer = [0u8; 32];
    file.read_exact(&mut buffer)?;

    let mut hash = String::with_capacity(64);

    for byte in &buffer {
        write!(hash, "{:02x}", byte)?;
    }

    Ok(hash)
}
