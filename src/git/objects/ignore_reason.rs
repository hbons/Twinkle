//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::PathBuf;


#[derive(Debug)]
pub enum GitIgnoreReason {
    // Path to rules file, matched pattern
    GlobalGitIgnoreFile(PathBuf, String),
    SystemGitIgnore(PathBuf, String),
    LocalGitIgnoreFile(PathBuf, String),
    ExcludesFile(PathBuf, String),
}
