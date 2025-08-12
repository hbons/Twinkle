//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs::{ OpenOptions, create_dir_all };
use std::io::{ Read, Write };
use std::path::Path;

use serde_json;

use crate::git::objects::repository::GitRepository;
use crate::log;
use crate::twinkle::twinkle_pretty::twinkle_pretty_dir;


pub fn config_init(config_path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = config_path.parent() {
        create_dir_all(parent)?;
    }

    OpenOptions::new()
        .write(true)
        .create_new(true) // Error if file already exists
        .open(config_path)?;

    let path = twinkle_pretty_dir(config_path);
    log::debug(&format!("Config | Created `{}`", path));

    Ok(())
}


pub fn config_load(config_path: &Path) -> Result<Vec<GitRepository>, Box<dyn Error>> {
    let mut config = OpenOptions::new()
        .read(true)
        .create(false) // Error if file does not exist
        .open(config_path)?;

    let mut json = String::new();
    config.read_to_string(&mut json)?;

    let repos: Vec<GitRepository> = serde_json::from_str(&json)?;

    let count = repos.len();
    let path = twinkle_pretty_dir(config_path);
    log::debug(&format!("Config | Loaded {count} repos from `{path}`"));

    Ok(repos)
}


pub fn config_save(config_path: &Path, repos: Vec<GitRepository>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&repos)?;

    let mut config = OpenOptions::new()
        .write(true)
        .create(false) // Error if file does not exist
        .truncate(true)
        .open(config_path)?;

    config.write_all(json.as_bytes())?;

    let path = twinkle_pretty_dir(config_path);
    log::debug(&format!("Config | Saved to `{path}`"));

    Ok(())
}
