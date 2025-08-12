//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::str;


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum KeyType {
    #[default] ED25519, // TODO: ED25519(u32)
    RSA,
    ECDSA,
}


impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            KeyType::ECDSA   => "ecdsa-sha2-nistp256",
            KeyType::ED25519 => "ed25519",
            KeyType::RSA     => "rsa",
        })
    }
}


impl str::FromStr for KeyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ecdsa-sha2-nistp256" => Ok(KeyType::ECDSA),
            "ed25519"             => Ok(KeyType::ED25519),
            "rsa"                 => Ok(KeyType::RSA),
            _ => Err("Invalid key type".into()),
        }
    }
}


impl KeyType {
    // "ssh-ed25519 AAAAC3NzaC1lZDI1N… Twinkle"
    pub fn from_public_key(s: &str) -> Result<Self, String> {
        let key_type = s.split_whitespace().next().ok_or("Invalid public key string")?;
        let key_type = key_type.strip_prefix("ssh-").unwrap_or(key_type);

        key_type.parse::<KeyType>()
    }
}
