//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use notify::{
    Config,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use crate::core::objects::repository::TwinkleRepository;
use crate::git::objects::status_filter::GitStatusFilter;
use crate::log;


const LOCAL_INTERVAL: u64 = 60;

pub fn local( // TODO: Use tokio
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    loop {
        if !repo.is_busy() {
            if repo.git.status(GitStatusFilter::All).is_some() {
                repo.set_has_local_changes(true);
                log::debug("local | Changes detected…");
            }
        }

        thread::sleep(
            Duration::from_secs(LOCAL_INTERVAL)
        );
    }
}


const LOCAL_TIMEOUT_MS: u64 = 500;

pub fn local_using_notify(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let (send, recv) = channel();

    let mut watcher = RecommendedWatcher::new(send, Config::default())?;
    watcher.watch(&repo.path, RecursiveMode::Recursive)?;

    let mut prev_path = PathBuf::new();
    let timeout = Duration::from_millis(LOCAL_TIMEOUT_MS);

    loop {
        if let Ok(Ok(event)) = recv.recv_timeout(timeout) {
            if repo.is_busy() {
                continue;
            }

            for path in event.paths {
                if path == prev_path || should_ignore(&path) {
                    continue;
                }

                log::debug(&format!("local-notify | Event: `{}`", path.display()));
                repo.set_has_local_changes(true);

                prev_path = path;
            }
        }
    }
}


fn should_ignore(path: &Path) -> bool {
    path.components()
        .any(|c| {
            let s = c.as_os_str();

            s == ".git" ||
            s == ".DS_Store"
        })
}
