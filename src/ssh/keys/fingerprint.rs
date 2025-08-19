//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;


#[derive(Clone, Debug)]
pub enum Fingerprint {
    SHA256(String),
}


impl str::FromStr for Fingerprint {
    type Err = Box<dyn Error<>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (algorithm, hash) = s.split_once(':').ok_or("Missing ':'")?;

        match algorithm {
            "SHA256" => {
                if hash.len() == 43 {
                    Ok(Self::SHA256(hash.into()))
                } else {
                    Err("Hash must be 43 characters long'".into())
                }
            },
            _ => Err("Fingerprint must start with algorithm, e.g. 'SHA256:'".into())
        }
    }
}


impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Fingerprint::SHA256(ref hash) => write!(f, "SHA256:{}", hash),
        }
    }
}
