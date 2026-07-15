//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::consts::OS;
use std::error::Error;
use std::thread;
use std::time::Duration;

use chrono::Utc;

use crate::log;
use crate::ssh::util::ssh_util_test_connection;
use crate::twinkle::twinkle_init::init_id;

use super::objects::repository::TwinkleRepository;
use super::defaults::common::twinkle_default_sync_up_delay_max;
use super::defaults::common::twinkle_default_sync_up_delay_bump;
use super::twinkle_init::twinkle_init_common;
use super::twinkle_keys::twinkle_hostkey_for;
use super::twinkle_lfs::twinkle_lfs_track;
use super::twinkle_notify::twinkle_notify;
use super::twinkle_resolve::twinkle_resolve_changes;
use super::twinkle_pretty::twinkle_pretty_commit_message;
use super::twinkle_util::twinkle_ssh_command;


pub fn twinkle_sync_prepare(
    repo: &mut TwinkleRepository
) -> Result<(), Box<dyn Error>>
{
    // TODO: Error on invalid config. --local or --file

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
        let host_key = twinkle_hostkey_for(
            &repo.remote_url().ok_or("Missing remote_url")?,
            key_pair.key_type,
            key_pair.private_key_path.parent().ok_or("No parent")?
        )?;

        twinkle_init_common(repo, Some(key_pair))?;

        repo.set_user(&user)?;
        repo.set_user_signing_key(key_pair)?;
        repo.set_commit_gpg_sign(true)?;
        repo.set_tag_gpg_sign(true)?;

        repo.git.GIT_SSH_COMMAND = twinkle_ssh_command(Some(key_pair));

        let remote_url = repo.remote_url().ok_or("Missing remote_url")?;
        ssh_util_test_connection(&remote_url, &host_key, key_pair)?;

        log::debug(&format!("✓ Authenticated to {}", &remote_url.host));
    }

    Ok(())
}


pub fn twinkle_sync(
    repo: &mut TwinkleRepository,
    interval: Option<Duration>,
    once: bool,
) -> Result<(), Box<dyn Error>>
{
    twinkle_sync_prepare(repo)?;

    let repo_c1 = repo.clone();
    let repo_c2 = repo.clone();
    let mut repo_c3 = repo.clone();
    thread::spawn(move || { _ = twinkle_notify(&repo_c1); });
    thread::spawn(move || { _ = twinkle_watch_local(&repo_c2); });
    thread::spawn(move || { _ = twinkle_watch_remote(&mut repo_c3, interval); });

    if twinkle_has_unpushed_commits(repo) {
        repo.set_has_local_changes(true);
    }

    let mut start_sync = false;

    // This is the main loop
    loop {
        if repo.has_local_changes() || repo.has_remote_changes() {
            start_sync = true;
        }

        if !start_sync {
            thread::sleep(Duration::from_millis(500));
            continue;
        }

        repo.set_is_busy(true);

        if repo.has_local_changes() {
            // TODO: Add (small) configurable delay)

            match twinkle_sync_up(repo) {
                Ok(_) => {
                    repo.set_has_local_changes(false);
                    repo.set_last_synced(Utc::now().timestamp())?;
                },
                Err(e) => log::error(&e.to_string()),
            }
        }

        if repo.has_remote_changes() {
            match twinkle_sync_down(repo) {
                Ok(_) => {
                    repo.set_has_remote_changes(false);
                    repo.set_last_synced(Utc::now().timestamp())?;
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


pub fn twinkle_watch_local(repo: &TwinkleRepository) -> Result<(), Box<dyn Error>> {
    loop {
        if !repo.is_busy() {
            let status = repo.git.status()?;
            if !status.is_empty() {
                repo.set_has_local_changes(true);
                log::info("Local changes detected…");
            }
        }

        thread::sleep(Duration::from_secs(3 * 60)); // TODO: Make configurable
    }
}


pub fn twinkle_watch_remote(repo: &mut TwinkleRepository, interval: Option<Duration>) -> Result<(), Box<dyn Error>> {
    loop {
        let interval = interval.unwrap_or(
            repo.polling_interval()
        );

        if !repo.is_busy() {
            let branch = repo.branch().ok_or("Not on a branch")?;

            if let Ok(remote_id) = repo.git.ls_remote(&branch) {
                if !repo.git.merge_base(&remote_id, &branch)? {
                    repo.set_has_remote_changes(true);
                    log::info("Remote changes detected…");
                }
            }

            repo.set_last_checked(Utc::now().timestamp())?;
        }

        thread::sleep(interval);
    }
}


pub fn twinkle_sync_up(repo: &mut TwinkleRepository) -> Result<(), Box<dyn Error>> {
    let mut attempts = 0;

    loop {
        init_id(repo)?;

        repo.git.lfs_config_filters(
            Some(repo.git.GIT_SSH_COMMAND.clone())
        )?;

        let status = repo.git.status()?;
        let lfs_enabled = repo.lfs_enabled();

        // TODO: loop this, but status() needs to return None when there are no more unstaged changes (status_y)
        // TODO: need a separate command to check any (staged or unstaged) changes. remove plain status(). status_staged()+status_unstaged()+status_staged_or_unstaged()?
        for change in status {
            if lfs_enabled {
                _ = twinkle_lfs_track(repo, &change);
            }

            _ = repo.git.add(&change.path); // TODO: error get eaten and may cause an infinite loop
        }

        let status = repo.git.status()?; // TODO: status_staged()

        if let Some(message) = twinkle_pretty_commit_message(&status) {
            let user = repo.user().ok_or("User not set")?;

            repo.set_user(&user)?;
            repo.git.commit(Some(user), &message)?;

            log::info(&format!("✓ Committed. Now at {}", repo.current_head()?));
        } else {
            log::info(&format!("Nothing new to commit. Still at {}", repo.current_head()?));
            return Ok(()); // TODO: Also check unpushed commits
        }

        if repo.read_only() {
            return Ok(());
        }

        let branch = repo.branch().ok_or("Not on a branch")?;

        repo.git.lfs_install_pre_push_hook(Some(repo.git.GIT_SSH_COMMAND.clone()))?;
        let push = repo.git.push("origin", &branch);

        match push {
            Ok(_)  => log::info(&format!("✓ Pushed. Local and remote at {}", repo.current_head()?)),
            Err(_) => {
                log::info("✗ Push failed. Fetching…");
                let fetch = twinkle_sync_down(repo);

                if fetch.is_err() {
                    attempts += 1;
                    thread::sleep(twinkle_sync_up_delay(attempts));
                }
            }
        }

        let status = repo.git.status()?;
        if !twinkle_has_unpushed_commits(repo) && status.is_empty() {
            break;
        }
    }

    Ok(())
}


pub fn twinkle_sync_up_delay(attempts: u64) -> Duration {
    let max  = twinkle_default_sync_up_delay_max().as_secs();
    let bump = twinkle_default_sync_up_delay_bump().as_secs();

    let delay = (attempts * bump).saturating_sub(bump).min(max);
    log::info(&format!("Retrying in {}s…", delay));

    Duration::from_secs(delay)
}


pub fn twinkle_sync_down(repo: &mut TwinkleRepository) -> Result<(), Box<dyn Error>> {
    repo.git.fetch("main")?;

    if repo.lfs_enabled() {
        repo.git.lfs_fetch()?;
    }

    if OS == "macos" { repo.git.config_set("core.ignoreCase", "true")?; }
    let merge = repo.git.merge("FETCH_HEAD");

    if merge.is_err() {
        twinkle_resolve_changes(repo)?;
    }

    if OS == "macos" { repo.git.config_set("core.ignoreCase", "false")?; }

    log::info(&format!("✓ Fetched and merged. Now at {}", repo.current_head()?));
    Ok(())
}


pub fn twinkle_has_unpushed_commits(repo: &TwinkleRepository) -> bool {
    match repo.git.rev_list_count() {
        Ok(count) => count > 0,
        Err(_) => false,
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
