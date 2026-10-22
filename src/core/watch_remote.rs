//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use tokio::sync::Mutex;

use crate::core::objects::repository::TwinkleRepository;
use crate::core::objects::push_connection::PushConnection;
use crate::log;


pub fn remote(
    repo: &TwinkleRepository,
    interval: Option<Duration>,
) -> Result<(), Box<dyn Error>>
{
    loop {
        let interval = interval.unwrap_or(
            repo.polling_interval()
        );

        if !repo.is_busy() {
            if let Ok(true) = has_unfetched_commits(repo) {
                repo.set_has_remote_changes(true);
            }

            let now = Utc::now().timestamp();
            repo.set_last_checked(now)?;
        }

        // TODO: Compare to repo.last_checked() so
        // we can detect wake from sleep and start
        // checking remote immediately
        thread::sleep(interval);
    }
}


pub async fn remote_using_push(
    repo: Arc<TwinkleRepository>,
    connection: Arc<Mutex<PushConnection>>,
) -> Result<(), Box<dyn Error>>
{
    tokio::spawn(async move {
        let mut connection = connection.lock().await;

        if let Some(channel) = repo.channel() {
            _ = connection.listen(&channel, repo).await;
        }
    });

    Ok(())
}


fn has_unfetched_commits(
    repo: &TwinkleRepository,
) -> Result<bool, Box<dyn Error>>
{ // TODO: Move to Repository
    let branch = repo.branch().ok_or("Not on a branch")?;
    let remote = repo.remote(&branch);

    if let Ok(remote_id) = repo.git.ls_remote(&remote, &branch) {
        if !repo.git.merge_base(&remote_id, &branch)? {
            repo.set_has_remote_changes(true);
            log::debug("✓ Unfetched commits found");

            return Ok(true);
        }
    }

    Ok(false)
}
