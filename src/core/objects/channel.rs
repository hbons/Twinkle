//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::str;

use sha2::{ Digest, Sha256 };


pub struct TwinkleChannel(String);


impl str::FromStr for TwinkleChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TwinkleChannel(
            Sha256::digest(s)
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect()
            )
        )
    }
}


impl fmt::Display for TwinkleChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
