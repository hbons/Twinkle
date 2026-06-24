//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;
use chrono::{ DateTime, Local };

use crate::git::objects::change::GitChange;
use crate::git::objects::file_status::GitFileStatus;


/// "/Users/hbons/Projects" -> "~/Projects"
pub fn twinkle_pretty_dir(dir: &Path) -> String {
    let home_dir = std::env::var("HOME").unwrap_or("".to_string());
    let dir = dir.to_string_lossy().to_string();

    if dir.starts_with(&home_dir) {
        dir.replace(&home_dir, "~")
    } else {
        dir.to_string()
    }
}


/// true / false -> "Yes" / "No"
pub fn twinkle_pretty_bool(value: bool) -> &'static str {
    match value {
        true  => "Yes".trim(),
        false => "No ".trim(),
    }
}


/// 0 -> "1970-01-01 01:00:00 +01:00"
pub fn twinkle_pretty_datetime(seconds_from_epoch: i64) -> String {
    if let Some(datetime) = DateTime::from_timestamp(seconds_from_epoch, 0) {
        let local_time: DateTime<Local> = datetime.with_timezone(&Local);
        local_time.to_string()
    } else {
        "Is time even real?".to_string()
    }
}


// '+10, ~7, -3'
// '~ "README.md"'
pub fn twinkle_pretty_commit_message(status: &Vec<GitChange>) -> Option<String> {
    let (mut added, mut modified, mut deleted) = (0, 0, 0);
    let mut file = String::new();

    for change in status {
        match change.status_x {
            Some(GitFileStatus::Added)       => { added += 1; },
            Some(GitFileStatus::Modified)    => { modified += 1; },
            Some(GitFileStatus::Deleted)     => { deleted += 1; },
            Some(GitFileStatus::Renamed(_))  => { deleted += 1; added += 1; },
            Some(GitFileStatus::Copied(_))   => { added += 1; },
            _ => ()
        };

        file = change.path.to_string_lossy().to_string();
    }

    match added + modified + deleted {
        0 => None,
        1 if added    == 1 => Some(format!("+ \"{file}\"")),
        1 if modified == 1 => Some(format!("~ \"{file}\"")),
        1 if deleted  == 1 => Some(format!("− \"{file}\"")),
        _ => {
            let mut message = Vec::new();
            if added    > 0 { message.push(format!("+{added}")); }
            if modified > 0 { message.push(format!("~{modified}")); }
            if deleted  > 0 { message.push(format!("−{deleted}")); }

            Some(message.join(", "))
        }
    }
}
