//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;

use serde::{ Deserialize, Serialize };

use crate::ssh::keys::key_pair::KeyPair;


#[derive(Clone, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct GitUser {
    pub name: GitUserName,
    pub email: GitUserEmail,

    #[serde(skip)]
    pub key_pair: Option<KeyPair>,
}


impl GitUser {
    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn email(&self) -> &str {
        &self.email.as_str()
    }

    pub fn key_pair(&self) -> &Option<KeyPair> {
        &self.key_pair
    }
}


impl str::FromStr for GitUser {
    type Err = Box<dyn Error>;

    // 'Hylke Bons <hi@planetpeanut.uk>'
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, rest) = line.split_once('<').ok_or("Missing '<'")?;
        let email = rest.strip_suffix('>').ok_or("Missing '>'")?;

        if name.is_empty() || email.is_empty() {
            return Err("Missing name or email".into());
        }

        let user = GitUser {
            name:  GitUserName::new(name.into())?,
            email: GitUserEmail::new(email.into())?,
            key_pair: None,
        };

        Ok(user)
    }
}


impl fmt::Display for GitUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name.as_str(), self.email.as_str())
    }
}


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GitUserName(String);

impl GitUserName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            Err("Name cannot be empty".into())
        } else {
            Ok(Self(name.trim().into()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for GitUserName {
    fn default() -> Self {
        GitUserName("Unknown".to_string())
    }
}


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GitUserEmail(String);

impl GitUserEmail {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Self(email.trim().into()))
        } else {
            Err("Invalid email address".into())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for GitUserEmail {
    fn default() -> Self {
        GitUserEmail("git@localhost".to_string())
    }
}
