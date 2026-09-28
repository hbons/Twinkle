//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str;


#[derive(Debug, PartialEq, Eq)]
pub struct GitId(String);


impl GitId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


impl str::FromStr for GitId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();

        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid characters".into());
        }

        match s.len() {
            64 => Ok(Self(s.into())), // SHA256
            40 => Ok(Self(s.into())), // SHA1
            _ => Err("Invalid id length".into()),
        }
    }
}
