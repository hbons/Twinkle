//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::Path;

use chrono::{ DateTime, Local };

use crate::cli::util;
use crate::git::objects::change::GitChange;
use crate::git::objects::reference::GitReference;
use crate::git::objects::status::GitFileStatus;
use crate::ssh::objects::url::SshUrl;


/// "/Users/hbons/Projects" -> "~/Projects"
pub fn format_dir(dir: &Path) -> String {
    let home_dir = env::var("HOME")
        .unwrap_or("".to_string());

    let dir = dir
        .to_string_lossy()
        .to_string();

    if dir.starts_with(&home_dir) {
        dir.replace(&home_dir, "~")
    } else {
        dir
    }
}


/// true / false -> "Yes" / "No"
pub fn format_bool(value: bool) -> &'static str {
    match value {
        true  => "Yes".trim(),
        false => "No ".trim(),
    }
}


/// 0 -> "1970-01-01 01:00:00 +01:00"
pub fn format_datetime(seconds_from_epoch: i64) -> String {
    if let Some(datetime) = DateTime::from_timestamp(seconds_from_epoch, 0) {
        let local_time: DateTime<Local> = datetime.with_timezone(&Local);
        local_time.to_string()
    } else {
        "Is time even real?".into()
    }
}


// '+10, ~7, -3'
// '~ `README.md`'
pub fn format_commit_message(changes: &[GitChange]) -> Option<String> {
    let (mut added, mut modified, mut deleted) = (0, 0, 0);
    let mut file = String::new();

    for change in changes {
        match change.status_x {
            Some(GitFileStatus::Added)       => { added += 1; },
            Some(GitFileStatus::Modified)    => { modified += 1; },
            Some(GitFileStatus::Deleted)     => { deleted += 1; },
            Some(GitFileStatus::Renamed(_))  => { deleted += 1; added += 1; },
            Some(GitFileStatus::Copied(_))   => { added += 1; },
            _ => ()
        };

        file = change.path
            .to_string_lossy()
            .to_string();
    }

    match added + modified + deleted {
        0 => None,
        1 if added    == 1 => Some(format!("+ `{file}`")),
        1 if modified == 1 => Some(format!("~ `{file}`")),
        1 if deleted  == 1 => Some(format!("− `{file}`")),
        _ => {
            let mut message = Vec::new();
            if added    > 0 { message.push(format!("+{added}")); }
            if modified > 0 { message.push(format!("~{modified}")); }
            if deleted  > 0 { message.push(format!("−{deleted}")); }

            Some(message.join(", "))
        }
    }
}


// github.com:hbons/notes | 5cce3 | A `TWINKLE.md`
pub fn format_repo_change(
    url: &SshUrl,
    _branch: &GitReference,
    hash: &str, // TODO: GitId
    change: &GitChange,
) -> Option<String>
{
    let status = change.status_x.clone()?;
    let letter = format_file_status(&status);

    let host = url.to_string_alternate();
    let host = host
        .strip_prefix("git@")
        .unwrap_or(&host);

    Some(format!("{host} | {hash} | {letter} `{}`", change.path.display()))
}


pub fn format_file_status(status: &GitFileStatus) -> String {
    let s = status.to_string();

    match status {
        GitFileStatus::Added       => util::cli_green(&s),
        GitFileStatus::Copied(_)   => util::cli_green(&s),
        GitFileStatus::Modified    => util::cli_yellow(&s),
        GitFileStatus::Renamed(_)  => util::cli_yellow(&s),
        GitFileStatus::TypeChanged => util::cli_yellow(&s),
        GitFileStatus::Unmerged    => util::cli_red(&s),
        GitFileStatus::Deleted     => util::cli_red(&s),
        GitFileStatus::Ignored     => s,
        GitFileStatus::Untracked   => s,
    }
}
