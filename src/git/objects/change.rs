//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::{ OsStr, OsString };
use std::str;
use std::os::unix::ffi::{ OsStrExt, OsStringExt };
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


impl GitChange {
    // 'A	"src/git.rs"'
    // 'R100	"src/git.rs"	"src/git stuff.rs"'
    pub fn from_log_line_old(line: &str) -> Result<GitChange, Box<dyn Error>> {
        let mut parts = line.split('\t');

        let status_x = parts.next().ok_or("Missing status code X")?;
        let status_x = status_x.parse::<GitFileStatus>().ok();

        if status_x.is_none() {
            return Err("Missing status code X".into())
        }

        // if status_x ==

        let path = parts.next().ok_or("Error parsing change path")?;
        let path = PathBuf::from(path); // Self::strip_path_quotes(path);

        let mut change = GitChange {
            status_x: status_x.clone(),
            status_y: None,
            path,
        };

        if let Some(s) = parts.next() {
            let orig_path = change.path.clone();
            let status = Self::wrap_orig_path(&orig_path, status_x).ok_or("err")?;

            change.status_x = Some(status);
            change.path = PathBuf::from(s); // Self::strip_path_quotes(s);
        }

        Ok(change)
    }


    // 'A^@src/git.rs'
    // 'R100^@src/git.rs^@src/git stuff.rs'
    pub fn from_log_line(_line: &OsStr) -> Result<GitChange, Box<dyn Error>> {


        // TODO


        Ok(GitChange::default())
    }


    // 'A  src/main.rs'
    // ' D src/main.rs'
    // 'R  src/main.rs^@src/main stuff.rs'
    // ' C src/main.rs^@src/main stuff.rs'
    pub fn from_status_line(line: &OsStr) -> Result<GitChange, Box<dyn Error>> {
        let mut change = GitChange::default();

        let (status_x, status_y) = {
            let mut iter = line.as_bytes().iter();

            // First 3 chars are always ASCII (1-byte)
            let x = iter.next().ok_or("Missing status code X")?;
            let y = iter.next().ok_or("Missing status code Y")?;
            let _space = iter.next().ok_or("Missing space")?;

            (format!("{}", *x as char).parse::<GitFileStatus>().ok(),
             format!("{}", *y as char).parse::<GitFileStatus>().ok())
        };

        let chunks = line
            .as_bytes()
            .split(|&b| b == 0)
            .filter(|line| !line.is_empty())
            .map(OsStr::from_bytes)
            .collect::<Vec<&OsStr>>();

        let mut iter = chunks.into_iter();

        change.status_x = status_x.clone();
        change.status_y = status_y.clone();
        change.path = {
            if let Some(chunk) = iter.next() {
                let bytes = chunk // SAFETY: Checked the 3 bytes above
                    .as_bytes()[3..]
                    .to_vec();

                Path::new(
                    &OsString::from_vec(bytes)
                ).to_path_buf()
            } else {
                PathBuf::new()
            }
        };

        if let Some(orig_path) = iter.next() {
            change.status_x = match status_x {
                Some(GitFileStatus::Renamed(_)) |
                Some(GitFileStatus::Copied(_)) => {
                    Self::wrap_orig_path(
                        Path::new(orig_path),
                        status_x,
                    )
                },
                _ => change.status_x,
            };

             change.status_y = match status_y {
                Some(GitFileStatus::Renamed(_)) |
                Some(GitFileStatus::Copied(_)) => {
                    Self::wrap_orig_path(
                        Path::new(orig_path),
                        status_y,
                    )
                },
                _ => change.status_y,
            }
        }

        Ok(change)
    }
}


impl GitChange {
    /// Wraps a path in a supported GitFileStatus enum
    fn wrap_orig_path(
        path: &Path,
        status: Option<GitFileStatus>,
    ) -> Option<GitFileStatus>
    {
        match status {
            Some(GitFileStatus::Renamed(_)) =>
                Some(GitFileStatus::Renamed(
                    Some(path.to_path_buf()))
                ),
            Some(GitFileStatus::Copied(_)) =>
                Some(GitFileStatus::Copied(
                    Some(path.to_path_buf()))
                ),
            _ => None,
        }
    }
}


impl GitChange {
    /// e.g. "D" and "U" -> GitMergeStatus::DU
    pub fn as_merge_status(&self) -> Option<GitMergeStatus> {
        match (&self.status_x, &self.status_y) {
            (Some(x), Some(y)) => format!("{x}{y}").parse::<GitMergeStatus>().ok(),
            _ => None,
        }
    }
}
