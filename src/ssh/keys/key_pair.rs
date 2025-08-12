//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::{ Path, PathBuf };

use super::key_type::KeyType;


#[derive(Clone, Debug, Default)]
pub struct KeyPair {
    pub key_type: KeyType,

    pub private_key_path: PathBuf,
    pub private_key: String,
    pub passphrase: Option<String>,

    pub public_key_path: PathBuf,
    pub public_key: String,
}


impl KeyPair {
    // ~/.config/twinkle/keys/github.com.ed25519.key
    // ~/.config/twinkle/keys/github.com.ed25519.key.pub
    // ~/.config/twinkle/keys/github.com.ed25519.key.host
    pub fn from_file(private_key_path: &Path) -> Result<Self, Box<dyn Error>> {
        let private_key_path = private_key_path.to_path_buf();
        let public_key_path  = private_key_path.with_extension("key.pub");

        let private_key = fs::read_to_string(&private_key_path)?;
        let public_key  = fs::read_to_string(&public_key_path)?;

        let key_pair = KeyPair {
            key_type: KeyType::from_public_key(&public_key)?,

            private_key_path,
            private_key: private_key.trim().to_string(),
            passphrase: None,

            public_key_path,
            public_key: public_key.trim().to_string(),
        };

        Ok(key_pair)
    }
}
