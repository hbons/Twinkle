//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::process::Command;
use std::str;


#[derive(Clone, Debug, PartialEq)]
pub enum GitVersion {
    Git2(Option<String>),
    Git3(Option<String>),
}


impl GitVersion {
    // Docs: https://git-scm.com/docs/git-version

    pub fn new() -> Option<Self> {
        Command::new("git").arg("--version")
            .output()
            .ok()
            .map(|o| {
                let s = String::from_utf8_lossy(
                    o.stdout.trim_ascii_end()
                );

                s.parse::<Self>().unwrap_or_default()
            })
    }
}


impl fmt::Display for GitVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitVersion::Git2(Some(v)) |
            GitVersion::Git3(Some(v)) => write!(f, "git version {v}"),
            GitVersion::Git2(None) => write!(f, "git version 2"),
            GitVersion::Git3(None) => write!(f, "git version 3"),
        }
    }
}


impl str::FromStr for GitVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(v) = s.strip_prefix("git version 2.") {
            return Ok(Self::Git2(Some(format!("2.{v}"))));
        }

        if let Some(v) = s.strip_prefix("git version 3.") {
            return Ok(Self::Git3(Some(format!("3.{v}"))));
        }

        Err(format!("Could not parse Git version: {s}"))
    }
}


impl Default for GitVersion {
    fn default() -> Self {
        Self::Git2(None)
    }
}
