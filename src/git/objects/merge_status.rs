//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::str;


#[derive(Debug, PartialEq)]
pub enum GitMergeStatus {
    // Docs: https://git-scm.com/docs/git-status#_short_format

    /// unmerged, both deleted
    DD,
    /// unmerged, added by us
    AU,
    /// unmerged, deleted by them
    UD,
    /// unmerged, added by them
    UA,
    /// unmerged, deleted by us
    DU,
    /// unmerged, both added
    AA,
    /// unmerged, both modified
    UU,

    /// untracked ("??")
    QQ, //
    /// untracked and ignored ("!!")
    XX,
}


impl str::FromStr for GitMergeStatus {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DD" => Ok(GitMergeStatus::DD),
            "AU" => Ok(GitMergeStatus::AU),
            "UD" => Ok(GitMergeStatus::UD),
            "UA" => Ok(GitMergeStatus::UA),
            "DU" => Ok(GitMergeStatus::DU),
            "AA" => Ok(GitMergeStatus::AA),
            "UU" => Ok(GitMergeStatus::UU),
            "??" => Ok(GitMergeStatus::QQ),
            "!!" => Ok(GitMergeStatus::XX),
            _ => Err(format!("Invalid merge status: {}", s).into()),
        }
    }
}


impl fmt::Display for GitMergeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            GitMergeStatus::DD => "DD",
            GitMergeStatus::AU => "AU",
            GitMergeStatus::UD => "UD",
            GitMergeStatus::UA => "UA",
            GitMergeStatus::DU => "DU",
            GitMergeStatus::AA => "AA",
            GitMergeStatus::UU => "UU",
            GitMergeStatus::QQ => "??",
            GitMergeStatus::XX => "!!",
        };
        write!(f, "{}", status)
    }
}
