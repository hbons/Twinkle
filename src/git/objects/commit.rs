//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use chrono::{ DateTime, Utc };

use super::change::GitChange;
use super::commit_message::GitCommitMessage;
use super::user::GitUser;


#[derive(Debug, Default)]
pub struct GitCommit {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub author: GitUser,
    pub message: GitCommitMessage,
    pub changes: Vec<GitChange>,
}
