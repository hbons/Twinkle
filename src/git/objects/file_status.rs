//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::str;


#[derive(Clone, Debug, PartialEq)]
pub enum GitFileStatus {
    // Docs: https://git-scm.com/docs/git-status#_short_format

    Added,
    Modified,
    Deleted,
    /// Holds ORIG_PATH
    Renamed(PathBuf),
    /// Holds ORIG_PATH
    Copied(PathBuf),
    TypeChanged,
    Unmerged,
    Untracked,
    Ignored,
}


impl str::FromStr for GitFileStatus {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // C and R (sometimes M) may be followed by a 3-digit similarity score
        match s.chars().next() {
            Some('A') => Ok(GitFileStatus::Added),
            Some('M') => Ok(GitFileStatus::Modified),
            Some('D') => Ok(GitFileStatus::Deleted),
            Some('R') => Ok(GitFileStatus::Renamed(PathBuf::new())),
            Some('C') => Ok(GitFileStatus::Copied(PathBuf::new())),
            Some('T') => Ok(GitFileStatus::TypeChanged),
            Some('U') => Ok(GitFileStatus::Unmerged),
            Some('?') => Ok(GitFileStatus::Untracked),
            Some('!') => Ok(GitFileStatus::Ignored),
            _ => Err("Invalid file status".into()),
        }
    }
}


impl fmt::Display for GitFileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GitFileStatus::Added       => "A",
            GitFileStatus::Modified    => "M",
            GitFileStatus::Deleted     => "D",
            GitFileStatus::Renamed(_)  => "R",
            GitFileStatus::Copied(_)   => "C",
            GitFileStatus::TypeChanged => "T",
            GitFileStatus::Unmerged    => "U",
            GitFileStatus::Untracked   => "?",
            GitFileStatus::Ignored     => "!",
        })
    }
}
