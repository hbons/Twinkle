//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use crate::git::objects::repository::GitRepository;
use crate::git::objects::file_status::GitFileStatus;
use crate::git::objects::change::GitChange;

use crate::log;
use crate::twinkle::twinkle_default::twinkle_default_lfs_threshold;


pub fn twinkle_lfs_track(repo: &GitRepository, change: &GitChange) -> Result<(), Box<dyn Error>> {
    if change.status_x != Some(GitFileStatus::Untracked) &&
       change.status_y != Some(GitFileStatus::Untracked) &&
       change.status_x != Some(GitFileStatus::Added) &&
       change.status_y != Some(GitFileStatus::Added) {
        return Err("Nothing to track".into())
    }

    let threshold = repo.lfs_threshold
        .unwrap_or(twinkle_default_lfs_threshold());

    if repo.size_of(&change.path) >= Some(threshold) {
        _ = repo.git.lfs_track(&change.path);
        log::info(&format!("Tracking with LFS: `{}`", &change.path.to_string_lossy()));
    }

    Ok(())
}
