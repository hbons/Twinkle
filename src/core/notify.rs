//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{
    Config,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use crate::core::objects::repository::TwinkleRepository;
use crate::log;


const NOTIFY_TIMEOUT_MS: u64 = 500;


pub fn watch(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let (sender, receiver) = channel();

    let mut watcher = RecommendedWatcher::new(sender, Config::default())?;
    watcher.watch(&repo.path, RecursiveMode::Recursive)?;

    let mut prev_path = PathBuf::new();
    let timeout = Duration::from_millis(NOTIFY_TIMEOUT_MS);

    loop {
        if let Ok(Ok(event)) = receiver.recv_timeout(timeout) {
            if repo.is_busy() {
                continue;
            }

            for path in event.paths {
                if path == prev_path || should_ignore(&path) {
                    continue;
                }

                log::debug(&format!("Notify | Event: `{}`", path.to_string_lossy()));
                repo.set_has_local_changes(true);

                prev_path = path;
            }
        }
    }
}


fn should_ignore(path: &Path) -> bool {
    path.components()
        .any(|c| c.as_os_str() == ".git")
}
