//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::{ OsStr, OsString };
use std::os::unix::ffi::{ OsStrExt, OsStringExt };
use std::path::{ Path, PathBuf };

use super::status::GitFileStatus;
use super::status::GitMergeStatus;


#[derive(Debug, Default, PartialEq)]
pub struct GitChange {
    // Docs: https://git-scm.com/docs/git-status#_short_format

    pub status_x: Option<GitFileStatus>,
    pub status_y: Option<GitFileStatus>,

    pub path: PathBuf,
}


impl GitChange {
    // 'A\0src/git.rs'
    // 'R100\0src/git_orig.rs^@src/git_new.rs'
    pub fn from_log_line(line: &OsStr) -> Result<GitChange, Box<dyn Error>> {
        let mut change = GitChange::default();

        let status_x = {
            let mut iter = line.as_bytes().iter();

            // First char is always ASCII (1-byte)
            let x = iter.next().ok_or("Missing status code X")?;
            format!("{}", *x as char).parse::<GitFileStatus>().ok()
        };

        if status_x.is_none() {
            return Err("Missing status code X".into());
        }

        change.status_x = status_x.clone();

        let mut iter = line
            .as_bytes()
            .split(|&b| b == 0)
            .filter(|line| !line.is_empty())
            .map(OsStr::from_bytes)
            .collect::<Vec<&OsStr>>()
            .into_iter();

        // Compared to git-status, path order is reversed:
        // ORIG_PATH first, PATH second
        if let Some(orig_path) = iter.nth(1) {
            change.status_x =
                match status_x {
                    Some(GitFileStatus::Renamed(_)) |
                    Some(GitFileStatus::Copied(_)) => {
                        Self::wrap_orig_path(
                            Path::new(orig_path),
                            status_x,
                        )
                    },
                    _ => change.status_x,
            };

            change.path =
                match iter.next() {
                    Some(path) => PathBuf::from(path),
                    None => PathBuf::from(orig_path),
                };
        }

        Ok(change)
    }


    // 'A  src/main.rs'
    // ' D src/main.rs'
    // 'R  src/main_new.rs\0src/main_orig.rs'
    // ' C src/main_new.rs\0src/main_orig.rs'
    pub fn from_status_line(line: &OsStr) -> Result<GitChange, Box<dyn Error>> {
        let mut change = GitChange::default();

        let (status_x, status_y) = {
            let mut iter = line.as_bytes().iter();

            // First 3 chars are always ASCII (1-byte)
            let x = iter.next().ok_or("Missing status code X")?;
            let y = iter.next().ok_or("Missing status code Y")?;
            let space = iter.next().ok_or("Missing space")?;

            if *space != b' ' {
                return Err("Missing space".into());
            }

            (format!("{}", *x as char).parse::<GitFileStatus>().ok(),
             format!("{}", *y as char).parse::<GitFileStatus>().ok())
        };

        if status_x.is_none() &&
           status_y.is_none() {
               return Err("Missing status code X and Y".into());
        }

        change.status_x = status_x.clone();
        change.status_y = status_y.clone();

        let mut iter = line
            .as_bytes()
            .split(|&b| b == 0)
            .filter(|line| !line.is_empty())
            .map(OsStr::from_bytes)
            .collect::<Vec<&OsStr>>()
            .into_iter();

        // Compared to git-log, path order is reversed:
        // PATH first, ORIG_PATH second
        change.path = {
            if let Some(chunk) = iter.next() {
                let bytes = chunk // SAFETY: Checked the 3 bytes above
                    .as_bytes()[3..]
                    .to_vec();

                OsString::from_vec(bytes).into()
            } else {
                PathBuf::new()
            }
        };

        if let Some(orig_path) = iter.next() {
            change.status_x =
                match status_x {
                    Some(GitFileStatus::Renamed(_)) |
                    Some(GitFileStatus::Copied(_)) => {
                        Self::wrap_orig_path(
                            Path::new(orig_path),
                            status_x,
                        )
                    },
                    _ => change.status_x,
                };

            change.status_y =
                match status_y {
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
    /// e.g. GitFileStatus::D and GitFileStatus::U -> GitMergeStatus::DU
    pub fn as_merge_status(&self) -> Option<GitMergeStatus> {
        match (&self.status_x, &self.status_y) {
            (Some(x), Some(y)) => format!("{x}{y}").parse::<GitMergeStatus>().ok(),
            _ => None,
        }
    }
}
