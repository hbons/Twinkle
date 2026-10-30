//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


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
use tokio;
use tokio::time;

use crate::core::objects::repository::TwinkleRepository;
use crate::git::objects::status_filter::GitStatusFilter;
use crate::log;


const LOCAL_INTERVAL: u64 = 60; // TODO: Read from config

pub async fn using_polling(
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

        time::sleep(
            Duration::from_secs(LOCAL_INTERVAL)
        ).await;
    }
}


const LOCAL_TIMEOUT_MS: u64 = 500;

pub async fn using_notify(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let (send, recv) = channel();

    let mut watcher = RecommendedWatcher::new(send, Config::default())?;
    watcher.watch(&repo.path, RecursiveMode::Recursive)?;

    let mut prev_path = PathBuf::new();
    let timeout = Duration::from_millis(LOCAL_TIMEOUT_MS);

    loop {
        // TODO: Is this async?
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
