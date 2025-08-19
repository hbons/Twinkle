//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::str::FromStr;

use chrono::{ DateTime, Utc };

use super::objects::change::GitChange;
use super::objects::commit::GitCommit;
use super::objects::commit_message::GitCommitMessage;
use super::objects::environment::GitEnvironment;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-log

    pub fn log(&self, count: usize) -> Result<Vec<GitCommit>, Box<dyn Error>> {
        let output = self.run("log", &[
            &format!("--max-count={count}"),
            "--date=unix", // Seconds since epoch
            "--no-renames", // Show renames as separate 'D' and 'A' lines
            "--name-status", // List files with change type
            "--no-color",
            "--no-decorate", // Don't show the (tracking) branch
            "--no-merges",
        ])?;

        let mut first = true;
        let mut log = Vec::new();
        let mut commit = GitCommit::default();
        let mut message = String::new();

        for line in output.stdout.lines() {
            if line.starts_with("commit") && !first {
                commit.message = message.parse::<GitCommitMessage>()?;
                log.push(commit);

                commit = GitCommit::default();
                message = String::new();
            } else {
                first = false;
            }

            parse_line(line, &mut commit, &mut message)?;
        }

        // Don't forget the last commit
        commit.message = message.parse::<GitCommitMessage>()?;
        log.push(commit);

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
// R097	src/file.rs	"src/file 2.rs"
// M	src/file3.rs
// M	src/file4.rs

fn parse_line(line: &str, commit: &mut GitCommit, message: &mut String) -> Result<(), Box<dyn Error>> {
    match line {
        s if s.trim().is_empty() => {
            message.push('\n');
        },
        s if s.starts_with("commit") => {
            match parse_line_id(line) {
                Some(id) => commit.id = id,
                None => return Err("Error parsing commit id".into()),
            }
        },
        s if s.starts_with("Author:") => {
            let line = line.strip_prefix("Author:").ok_or("Error parsing author")?;
            commit.author = GitUser::from_str(line)?;
        },
        s if s.starts_with("Date:") => {
            match parse_line_timestamp(line) {
                Some(timestamp) => commit.timestamp = timestamp,
                None => return Err("Error parsing timestamp".into()),
            }
        },
        s if !s.starts_with(" ") => {
            let change = GitChange::from_str(line)?;
            commit.changes.push(change);
        },
        _ => {
            message.push_str(line.trim_start());
            message.push('\n');
        }
    }

    Ok(())
}


// 'commit ab83b62f5027c66be4826c73f07daeb25fd04219'
fn parse_line_id(line: &str) -> Option<String> {
    let id = line.strip_prefix("commit")?;
    let id = id.trim().to_string();

    Some(id)
}


// 'Date:   1742391616'
fn parse_line_timestamp(line: &str) -> Option<DateTime<Utc>> {
    let line = line.strip_prefix("Date:")?.trim();
    let seconds_from_epoch: i64 = line.parse().unwrap_or(0);
    let timestamp = DateTime::from_timestamp(seconds_from_epoch, 0)?;

    Some(timestamp)
}
