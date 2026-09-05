//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;
use std::str;

use crate::ssh::keys::key_pair::KeyPair;
use crate::git::objects::environment::GitEnvironment;


#[derive(Clone, Debug, Default)]
pub struct GitUser {
    pub name: GitUserName,
    pub email: GitUserEmail,

    pub key_pair: Option<KeyPair>,
}


impl GitUser {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn key_pair(&self) -> Option<KeyPair> {
        self.key_pair.clone()
    }
}


impl GitUser {
    pub fn from(path: &Path) -> Option<Self> {
        let git = GitEnvironment::new(path);

        format!("{} <{}>",
            git.config_get("user.name")?,
            git.config_get("user.email")?,
        ).parse::<GitUser>().ok()
    }
}


impl str::FromStr for GitUser {
    type Err = Box<dyn Error>;

    // 'Hylke Bons <hello@planetpeanut.studio>'
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim();

        let (name, rest) = line.split_once('<').ok_or("Missing '<'")?;
        let email = rest.strip_suffix('>').ok_or("Missing '>'")?;

        if name.is_empty() || email.is_empty() {
            return Err("Missing name or email".into());
        }

        let user = GitUser {
            name: GitUserName::new(name.into())?,
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
pub struct GitUserName(String);

impl GitUserName {
    pub fn new(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            Err("Name cannot be empty".into())
        } else {
            Ok(Self(s.trim().into()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for GitUserName {
    fn default() -> Self {
        GitUserName("???".to_string())
    }
}


#[derive(Clone, Debug)]
pub struct GitUserEmail(String);

impl GitUserEmail {
    pub fn new(s: String) -> Result<Self, String> {
        if s.contains('@') {
            Ok(Self(s.trim().into()))
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
