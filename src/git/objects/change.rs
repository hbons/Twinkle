//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str;
use std::path::{ Path, PathBuf };

use super::file_status::GitFileStatus;
use super::merge_status::GitMergeStatus;


#[derive(Debug, Default, PartialEq)]
pub struct GitChange {
    // Docs: https://git-scm.com/docs/git-status#_short_format

    pub status_x: Option<GitFileStatus>,
    pub status_y: Option<GitFileStatus>,
    pub path: PathBuf,
}


impl str::FromStr for GitChange {
    type Err = Box<dyn Error>;

    /// String can be a file line from git-log or git-status
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.contains('\t') {
            GitChange::from_log_line(line)
        } else {
            GitChange::from_status_line(line)
        }
    }
}


impl GitChange {
    // 'A	"src/git.rs"'
    // 'R100	"src/git.rs"	"src/git stuff.rs"'
    fn from_log_line(line: &str) -> Result<GitChange, Box<dyn Error>> {
        let mut parts = line.split('\t');

        let status_x = parts.next().ok_or("Missing status code X")?;
        let status_x = status_x.parse::<GitFileStatus>().ok();

        if status_x.is_none() {
            return Err("Missing status code X".into())
        }

        let path = parts.next().ok_or("Error parsing change path")?;
        let path = Self::strip_path_quotes(path);

        let mut change = GitChange {
            status_x: status_x.clone(),
            status_y: None,
            path,
        };

        if let Some(s) = parts.next() {
            let orig_path = change.path.clone();
            let status = Self::wrap_orig_path(orig_path, status_x)?;

            change.status_x = Some(status);
            change.path = Self::strip_path_quotes(s);
        }

        Ok(change)
    }


    // 'A  "src/main.rs"'
    // ' D "src/main.rs"'
    // 'RM "src/main.rs" -> "src/main stuff.rs"'
    fn from_status_line(line: &str) -> Result<GitChange, Box<dyn Error>> {
        let mut chars = line.chars();

        let status_x = chars.next().ok_or("Missing status code X")?;
        let status_x = status_x.to_string().parse::<GitFileStatus>().ok();

        let status_y = chars.next().ok_or("Missing status code Y")?;
        let status_y = status_y.to_string().parse::<GitFileStatus>().ok();

        if let Some(space) = chars.next() {
            if space != ' ' {
                return Err("Missing space".into());
            }
        }

        let line = chars.collect::<String>();

        let mut change = GitChange {
            status_x: status_x.clone(),
            status_y: status_y.clone(),
            path: Self::strip_path_quotes(&line),
        };

        if let Some((orig_path, path)) = line.split_once("->") {
            let orig_path = Self::strip_path_quotes(orig_path);
            let status = Self::wrap_orig_path(orig_path, status_x)?;

            change.status_x = Some(status);
            change.path = Self::strip_path_quotes(path);
        }

        Ok(change)
    }
}


impl GitChange {
    /// Strips paths as they may be quoted if containing "unusual" characters
    fn strip_path_quotes(path: &str) -> PathBuf {
        // Docs: https://git-scm.com/docs/git-config#Documentation/git-config.txt-corequotePath

        let path = path.trim().trim_matches('"');
        Path::new(path).to_path_buf()
    }


    /// Wraps a path with the GitFileStatus::Renamed/Copied enum
    fn wrap_orig_path(new_path: PathBuf, status: Option<GitFileStatus>) -> Result<GitFileStatus, Box<dyn Error>> {
        match status {
            Some(GitFileStatus::Renamed(_)) => Ok(GitFileStatus::Renamed(new_path)),
            Some(GitFileStatus::Copied(_))  => Ok(GitFileStatus::Copied(new_path)),
            _ => Err("GitFileStatus is not Renamed or Copied".into()),
        }
    }
}


impl GitChange {
    pub fn as_merge_status(&self) -> Option<GitMergeStatus> {
        match (&self.status_x, &self.status_y) {
            (Some(x), Some(y)) => {
                let status = format!("{x}{y}");
                status.parse::<GitMergeStatus>().ok()
            }
            _ => None,
        }
    }
}
