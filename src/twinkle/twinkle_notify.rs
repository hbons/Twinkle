//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{ Config, RecommendedWatcher, RecursiveMode, Result, Watcher };

use crate::log;
use crate::twinkle::objects::twinkle_repository::TwinkleRepository;


pub fn twinkle_notify(repo: &TwinkleRepository) -> Result<()> {
    let (sender, receiver) = channel();
    let mut watcher = RecommendedWatcher::new(sender, Config::default())?;

    watcher.watch(&repo.path, RecursiveMode::Recursive)?;

    loop {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(500)) {
            if repo.is_busy() {
                continue;
            }

            let mut prev_path = PathBuf::new();

            for path in event.unwrap().paths {
                if path.components().any(|c| c.as_os_str() == ".git") {
                    continue;
                }

                if path == prev_path {
                    continue;
                }

                log::debug(&format!("Notify | Detected a change: `{}`", path.to_string_lossy()));
                repo.set_has_local_changes(true);

                prev_path = path;
            }
        }
    }
}
