//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;
use chrono::{ DateTime, Local };


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
