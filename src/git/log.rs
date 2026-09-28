//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::{ OsStr, OsString };
use std::os::unix::ffi::OsStrExt;

use chrono::{ DateTime, Utc };

use crate::log;

use super::objects::change::GitChange;
use super::objects::commit::GitCommit;
use super::objects::commit_message::GitCommitMessage;
use super::objects::environment::GitEnvironment;
use super::objects::id::GitId;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-log

    /// Note: Data returned by the log functions are lossy UTF-8 for now
    pub fn log(
        &self,
        max: Option<i32>,
    ) -> Result<Vec<GitCommit>, Box<dyn Error>>
    {
        self.log_internal(None, max)
    }


    pub fn log_since(
        &self,
        id: &GitId,
    ) -> Result<Vec<GitCommit>, Box<dyn Error>>
    {
        self.log_internal(Some(&format!("{id}..")), None)
    }

    // pub fn log_since_merge(
    //     &self,
    // ) -> Result<Vec<GitCommit>, Box<dyn Error>>
    // {
    //     self.log_internal(Some("ORIG_HEAD.."), None)
    // }


    fn log_internal(
        &self,
        reference: Option<&str>,
        max: Option<i32>,
    ) -> Result<Vec<GitCommit>, Box<dyn Error>>
    {
        let output = self.run("log", &[
            OsStr::new("-z"), // Single line, NUL-separated
            OsStr::new("--date=unix"), // Seconds since epoch
            OsStr::new(&format!("--max-count={}", max.unwrap_or(i32::MAX))),
            OsStr::new("--name-status"), // List files with change type
            OsStr::new("--no-color"),
            OsStr::new("--no-decorate"), // Don't show the (tracking) branch
            OsStr::new("--no-merges"), // Merges don't have a --name-status
            OsStr::new(reference.unwrap_or("HEAD")),
        ])?;

        let mut log = Vec::new();

        let stdout = String::from_utf8_lossy(&output.stdout); // TODO: Don't convert, keep bytes/osstr

        let entries: Vec<&str> = stdout
            .split("\0\0") // Commits are separated by a double NUL
            .collect();

        for entry in &entries {
            let lines: Vec<&str> = entry
                .split('\n')
                .filter(|line| !line.is_empty()) // TODO: Needed? Could mess up message body
                .collect();

            let mut commit = GitCommit::default();
            let mut message = String::new();

            for line in lines {
                parse_line(OsStr::new(line), &mut commit, &mut message)?;
            }

            if !commit.changes.is_empty() {
                commit.message = message.parse::<GitCommitMessage>()?;
                log.push(commit);
            }
        }

        Ok(log)
    }
}


// commit ab83b62f5027c66be4826c73f07daeb25fd04219
// Author: Hylke Bons <hello@planetpeanut.studio>
// Date:   1740261217
//
//     Message title
//
//     Message body (optional and multiline)
//
// R097^@src/file.rs^@src/file 2.rs^@M^@src/file3.rs^@M^@src/file4.rs
fn parse_line(
    line: &OsStr,
    commit: &mut GitCommit,
    message: &mut String,
) -> Result<(), Box<dyn Error>>
{
    let lossy_line = &line
        .to_string_lossy()
        .to_string();

    match lossy_line {
        s if s.trim().is_empty() => {
            message.push('\n');
        },
        s if s.starts_with("commit") => {
            match parse_line_id(lossy_line) {
                Some(s) => commit.id = s.parse::<GitId>()?,
                None => return Err("Error parsing commit id".into()),
            }
        },
        s if s.starts_with("Merge:") => (),
        s if s.starts_with("Author:") => {
            let lossy_line = lossy_line.strip_prefix("Author:").ok_or("Error parsing author")?;
            commit.author = lossy_line.parse::<GitUser>()?;
        },
        s if s.starts_with("Date:") => {
            match parse_line_timestamp(lossy_line) {
                Some(timestamp) => commit.timestamp = timestamp,
                None => return Err("Error parsing timestamp".into()),
            }
        },
        s if !s.starts_with(" ") => {
            match parse_line_name_status(line) {
                Some(changes) => commit.changes = changes,
                None => return Err("Error parsing name status".into()),
            }
        },
        _ => {
            message.push_str(lossy_line.trim_start());
            message.push('\n');
        }
    }

    Ok(())
}


// 'commit ab83b62f5027c66be4826c73f07daeb25fd04219'
fn parse_line_id(line: &str) -> Option<&str> {
    Some(
        line
            .strip_prefix("commit")?
            .trim()
    )
}


// 'Date:   1742391616'
fn parse_line_timestamp(line: &str) -> Option<DateTime<Utc>> {
    let line = line.strip_prefix("Date:")?.trim();
    let seconds_from_epoch: i64 = line.parse().unwrap_or(0);
    let timestamp = DateTime::from_timestamp(seconds_from_epoch, 0)?;

    Some(timestamp)
}


// 'R097^@src/file.rs^@src/file 2.rs^@M^@src/file3.rs^@M^@src/file4.rs'
fn parse_line_name_status(line: &OsStr) -> Option<Vec<GitChange>> {
    let mut changes = Vec::<GitChange>::new();

    let chunks = line
        .as_bytes()
        .split(|&b| b == 0)
        .filter(|chunk| !chunk.is_empty())
        .map(OsStr::from_bytes)
        .collect::<Vec<&OsStr>>();

    let mut iter = chunks.iter();
    let mut buf = OsString::new();

    while let Some(&chunk) = iter.next() {
        let x_byte = chunk.as_bytes();
        let path = *iter.next().unwrap(); // TODO

        let bytes = [
            x_byte,
            b"\0", // NUL
            path.as_bytes(),
        ].concat();

        let chunk = OsStr::from_bytes(&bytes);
        let chunk = {
            if x_byte.starts_with(b"R") ||
               x_byte.starts_with(b"C") {
                if let Some(&orig_path) = iter.next() {
                    buf.clear();
                    buf.push(chunk);
                    buf.push(OsStr::new("\0"));
                    buf.push(orig_path);
                    &buf
                } else {
                    chunk
                }
            } else {
                chunk
            }
        };

        match GitChange::from_log_line(chunk) {
            Ok(change) => changes.push(change),
            Err(e) => log::error(
                &format!("{e}: `{}`", chunk.to_string_lossy())
            ),
        }
    }

    Some(changes)
}
