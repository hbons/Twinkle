//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::consts::OS;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tokio::time;
use tokio::sync::Mutex;

use crate::core::objects::push_connection::PushConnection;
use crate::git::objects::status::GitStatusFilter;
use crate::git::config::K_CORE_IGNORE_CASE;

use crate::log;
use crate::ssh;
use crate::core::init;

use super::objects::repository::TwinkleRepository;
use super::defaults::common::default_sync_up_delay_max;
use super::defaults::common::default_sync_up_delay_bump;
use super::keys;
use super::lfs;
use super::resolve;
use super::watch_local;
use super::watch_remote;
use super::pretty;
use super::push;
use super::util;


pub fn prepare(
    repo: &mut TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    if repo.lfs_enabled() &&
        repo.git.lfs_version().is_none() {
        return Err("Git LFS enabled but not installed".into());
    }

    if repo.branch().is_none() {
        return Err("Not on a branch".into());
    }

    if repo.id().is_none() {
        return Err("Missing ID".into());
    }

    let user = repo.user().ok_or("Missing user")?;

    if let Some(key_pair) = &user.key_pair {
        let host_key = keys::hostkey_for(
            &repo.remote_url().ok_or("Missing remote_url")?,
            key_pair.key_type,
            key_pair.private_key_path.parent().ok_or("No parent")?
        )?;

        init::init_common(repo, Some(key_pair))?;

        repo.set_user(&user)?;
        repo.set_user_signing_key(key_pair)?;
        repo.set_commit_gpg_sign(true)?;
        repo.set_tag_gpg_sign(true)?;

        repo.git.GIT_SSH_COMMAND = util::ssh_command(Some(key_pair));

        let remote_url = repo.remote_url().ok_or("Missing remote_url")?;
        ssh::util::test_connection(&remote_url, &host_key, Some(key_pair))?;

        log::debug(&format!("✓ Authenticated to {}", remote_url.host));
    }

    Ok(())
}


pub async fn start(
    repo: &mut TwinkleRepository,
    connection: Option<Arc<Mutex<PushConnection>>>,
    interval: Option<Duration>,
    once: bool,
) -> Result<(), Box<dyn Error>>
{
    prepare(repo)?;

    if once {
        repo.set_has_remote_changes(true);
    }

    if has_unpushed_commits(repo) ||
       repo.git.status(GitStatusFilter::All).is_some() {
        repo.set_has_local_changes(true);
    }


    let repo_c1 = repo.clone();
    let repo_c2 = repo.clone();
    let repo_c3 = repo.clone();

    // Local
    tokio::spawn(async move { _ = watch_local::using_polling(&repo_c1).await; });  // Reliable, but slow
    tokio::spawn(async move { _ = watch_local::using_notify(&repo_c2.clone()).await; });  // Fast, but less reliable

    // Remote
    tokio::spawn(async move { _ = watch_remote::using_polling(&repo_c3, interval).await; });  // Reliable, but slow

    if repo.push_enabled() {
        let repo = Arc::new(repo.clone());

        if let Some(ref connection) = connection {
            _ = watch_remote::using_push(
                Arc::clone(&repo),
                Arc::clone(connection),
            ).await;
        }
    }


    let mut start_sync = false;

    // This is the main loop
    loop {
        if repo.has_local_changes() || repo.has_remote_changes() {
            start_sync = true;
        }

        if !start_sync {
            time::sleep(Duration::from_millis(500)).await;
            continue;
        }

        repo.set_is_busy(true);

        if repo.has_local_changes() {
            // TODO: Add (small) configurable delay)

            match sync_up(repo).await {
                Ok(_) => {
                    repo.set_has_local_changes(false);

                    let now = Utc::now().timestamp();
                    repo.set_last_synced(now)?;

                    if repo.push_enabled() {
                        if let Some(ref connection) = connection {
                            _ = push::send_notify(
                                Arc::new(repo.clone()),
                                Arc::clone(connection),
                            ).await;
                        }
                    }
                },
                Err(e) => log::error(&e.to_string()),
            }
        }

        if repo.has_remote_changes() {
            match sync_down(repo).await {
                Ok(_) => {
                    repo.set_has_remote_changes(false);

                    let now = Utc::now().timestamp();
                    repo.set_last_synced(now)?;
                },
                Err(e) => log::error(&e.to_string()),
            }
        }

        repo.set_is_busy(false);
        start_sync = false;

        if once {
            return Ok(());
        }
    }
}


async fn sync_up(
    repo: &mut TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let mut attempt = 1;

    loop {
        log::debug(&format!("Attempt: {attempt}"));
        init::init_id(repo)?;

        let lfs_enabled = repo.lfs_enabled();

        if lfs_enabled {
            repo.git.lfs_config_set_filter(
                Some(repo.git.GIT_SSH_COMMAND.clone())
            )?;
        } else {
            _ = repo.git.lfs_config_unset_filter();
        }

        while let Some(status) = repo.git.status(GitStatusFilter::Unstaged) {
            for change in status {
                if lfs_enabled {
                    // Discard any errors (file may have been deleted)
                    _ = lfs::track(repo, &change);
                }

                _ = repo.git.add(&change.path); // TODO: error get eaten and may cause an infinite loop
            }

            time::sleep(Duration::from_millis(500)).await; // Allow file activity to settle
        } // TODO: Prevent infinite loop here

        let branch = repo.git.branch_show_current()?;
        let remote = repo.remote(&branch);
        let changes = repo.git.status(GitStatusFilter::Staged)
            .unwrap_or_default(); // We need an empty Vec over None for the next block

        if let Some(message) = pretty::format_commit_message(&changes) {
            let user = repo.user().ok_or("User not set")?; // TODO: Commit with default user?

            repo.set_user(&user)?;
            repo.git.commit(Some(user), &message)?;

            log::debug(&format!("✓ Committed to `{branch}`. Now at {}", repo.current_head()?));
        } else {
            if !has_unpushed_commits(repo) {
                log::debug(&format!("Nothing new to commit. Still at {}", repo.current_head()?));
                return Ok(());
            }

            log::debug("✓ Unpushed commits found");
        }

        if repo.read_only() {
            return Ok(());
        }

        if lfs_enabled {
            repo.git.lfs_install_pre_push_hook(
                Some(repo.git.GIT_SSH_COMMAND.clone())
            )?;
        } else {
            _ = repo.git.lfs_uninstall_pre_push_hook();
        }

        let unpushed_commits = repo.git
            .log_unpushed()? // we don't have an upstream here yet on empty init repos
            .into_iter()
            .rev();

        let push = repo.git.push(&remote, &branch);

        match push {
            Ok(_)  => {
                let url = repo.remote_url().ok_or("Missing remote url")?;

                for commit in unpushed_commits {
                    for change in commit.changes.iter().rev() {
                        if let Some(s) = pretty::format_repo_change(&url, &commit.id, commit.is_merge, change, "↑") {
                            println!("{s}");
                        }
                    }
                }
            },
            Err(_) => {
                // TODO: "Could not sync 7 local changes. Retrying in…"
                log::debug("✗ Push failed. Fetching…");
                let fetch = sync_down(repo).await;

                if fetch.is_err() { // TODO: Only delay on network errors?
                    let delay = sync_up_delay(attempt);
                    log::debug(&format!("Retrying in {}s…", delay.as_secs()));
                    time::sleep(delay).await;
                }
            }
        }

        let changes = repo.git.status(GitStatusFilter::All);

        if !has_unpushed_commits(repo) && changes.is_none() {
            break;
        }

        attempt += 1;
    }

    Ok(())
}


pub fn sync_up_delay(attempt: u64) -> Duration { // TODO: Ugly name and function
    let max  = default_sync_up_delay_max().as_secs();
    let bump = default_sync_up_delay_bump().as_secs();

    let attempt = attempt.saturating_sub(1);
    let delay = (attempt * bump).min(max);

    Duration::from_secs(delay)
}


async fn sync_down(repo: &mut TwinkleRepository) -> Result<(), Box<dyn Error>> {
    let branch = repo.git.branch_show_current()?;
    let remote = repo.remote(&branch);

    repo.git.fetch(&remote, &branch)?;

    if repo.lfs_enabled() {
        repo.git.lfs_fetch()?;
    }

    let fetched_commits = repo.git
        .log_fetched()?
        .into_iter()
        .rev();

    for commit in fetched_commits {
        let url = repo.remote_url()
            .ok_or("Missing remote url")?;

        for change in commit.changes.iter().rev() {
            if let Some(s) = pretty::format_repo_change(&url, &commit.id, commit.is_merge, change, "↓") {
                println!("{s}");
            }
        }
    }

    if OS == "macos" { repo.git.config_set(K_CORE_IGNORE_CASE, "true")?; }
    let merge = repo.git.merge(&"FETCH_HEAD".into());

    if merge.is_err() {
        resolve::resolve_changes(repo).await?;
    }

    if OS == "macos" { repo.git.config_set(K_CORE_IGNORE_CASE, "false")?; }

    log::debug(&format!("✓ Fetched and merged. Now at {}", repo.current_head()?));

    Ok(())
}


fn has_unpushed_commits(repo: &TwinkleRepository) -> bool { // TODO: Move to Repository
    match repo.git.rev_list_count_upstream() {
        Ok(count) => count > 0,
        Err(_) => true,
    }
}



// pub enum TwinkleStatus {
//     UpToDate(i64), //(last check)
//     Error(String),
//     UnpushedChanges,
// }

// pub enum TwinklePushError {
//     NoNetwork(String),
//     NoAuth(String),
//     RemoteAhead(String),
//     Unknown(String),
// }

pub enum TwinkleFetchError {
    NoNetwork(String),
    NoAuth(String),
    NoRepository(String),
    Unknown(String),
}
