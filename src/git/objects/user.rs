//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::str;

use serde::{ Deserialize, Serialize };

use crate::ssh::keys::key_pair::KeyPair;


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GitUser {
    name: String,
    email: String,
    signing_key: Option<String>,

    #[serde(skip)] _key_pair: Option<KeyPair>, // TODO
}


impl GitUser {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn signing_key(&self) -> &Option<String> {
        &self.signing_key
    }

    pub fn key_pair(&self) -> &Option<KeyPair> {
        &self._key_pair
    }
}


impl Default for GitUser {
    fn default() -> Self {
        GitUser {
            name: "Twinkle".into(),
            email: "twinkle@localhost".into(),
            _key_pair: None,
            signing_key: None,
        }
    }
}


impl str::FromStr for GitUser {
    type Err = String;

    // 'Hylke Bons <hi@planetpeanut.uk>'
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, rest) = line.split_once('<').ok_or("Missing '<'")?;
        let email = rest.strip_suffix('>').ok_or("Missing '>'")?;

        if name.is_empty() || email.is_empty() {
            return Err("Missing name or email".into());
        }

        let user = GitUser {
            name: name.trim().into(),
            email: email.trim().into(),
            _key_pair: None,
            signing_key: None,
        };

        Ok(user)
    }
}


impl fmt::Display for GitUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
