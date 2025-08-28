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
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::util::ssh_util_test_connection;

use super::objects::twinkle_repository::TwinkleRepository;
use super::twinkle_default::twinkle_default_init;
use super::twinkle_default::twinkle_default_polling_interval;
use super::twinkle_default::twinkle_default_sync_up_delay_max;
use super::twinkle_default::twinkle_default_sync_up_delay_bump;
use super::twinkle_keys::twinkle_hostkey_for;
use super::twinkle_lfs::twinkle_lfs_track;
use super::twinkle_resolve::twinkle_resolve_changes;
use super::twinkle_util::{ twinkle_commit_message, twinkle_ssh_command };


pub fn twinkle_watch(repo: &mut TwinkleRepository, interval: Option<u64>) -> Result<(), Box<dyn Error>> {
    if repo.git.branch_show_current()? != repo.branch {
        return Err(format!("Repository not on branch as set in config ({})", repo.branch).into())
    }

    let key_pair = repo.user.key_pair.clone();
    let key_pair = key_pair.ok_or("No user key pair")?;

    let host_key = twinkle_hostkey_for(&repo.remote_url, KeyType::default(),
        &key_pair.private_key_path.parent().ok_or("No parent")?)?;

    repo.git.GIT_SSH_COMMAND = twinkle_ssh_command(&key_pair);

    twinkle_default_init(repo)?;
    repo.git.config_set_user(&repo.user)?;
    repo.git.config_set_user_signing_key(&key_pair)?;

    ssh_util_test_connection(&repo.remote_url, &host_key, &key_pair)?;
    log::debug(&format!("✓ Authenticated to {}", &repo.remote_url.host));

    let repo_c1 = repo.clone();
    let mut repo_c2 = repo.clone();
    thread::spawn(move || { _ = twinkle_watch_local(&repo_c1); });
    thread::spawn(move || { _ = twinkle_watch_remote(&mut repo_c2, interval); });

    if twinkle_has_unpushed_commits(repo) {
        repo.set_has_local_changes(true);
    }

    let mut start_sync = false;

    loop {
        if repo.has_local_changes() || repo.has_remote_changes() {
            start_sync = true;
        }

        if !start_sync {
            thread::sleep(Duration::from_millis(500));
            continue;
        }

        repo.set_is_syncing(true);

        if repo.has_local_changes() {
            match twinkle_sync_up(repo) {
                Ok(_) => {
                    repo.set_has_local_changes(false);
                    repo.last_synced = Utc::now().timestamp();
                },
                Err(e) => log::error(&e.to_string()),
            }
        }

        if repo.has_remote_changes() {
            match twinkle_sync_down(repo) {
                Ok(_) => {
                    repo.set_has_remote_changes(false);
                    repo.last_synced = Utc::now().timestamp();
                },
                Err(e) => log::error(&e.to_string()),
            }
        }

        repo.set_is_syncing(false);
        start_sync = false;
    }
}


pub fn twinkle_watch_local(repo: &TwinkleRepository) -> Result<(), Box<dyn Error>> {
    loop {
        if !repo.is_syncing() {
            let status = repo.git.status()?;
            if !status.is_empty() {
                repo.set_has_local_changes(true);
                log::info("Local changes detected…");
            }
        }

        thread::sleep(Duration::from_secs(5)); // TODO: Change to 5m once FS watcher is set up
    }
}


pub fn twinkle_watch_remote(repo: &mut TwinkleRepository, interval: Option<u64>) -> Result<(), Box<dyn Error>> {
    loop {
        let interval = interval.unwrap_or(
            repo.polling_interval.unwrap_or(
                twinkle_default_polling_interval()));

        if !repo.is_syncing() {
            if let Ok(remote_id) = repo.git.ls_remote(&repo.branch) {
                if !repo.git.merge_base(&remote_id, &repo.branch)? {
                    repo.set_has_remote_changes(true);
                    log::info("Remote changes detected…");
                }
            }

            repo.last_checked = Utc::now().timestamp();
        }

        thread::sleep(Duration::from_secs(interval));
    }
}


pub fn twinkle_sync_up(repo: &mut TwinkleRepository) -> Result<(), Box<dyn Error>> {
    let mut attempts = 0;

    loop {
        repo.git.lfs_config_filters()?;
        repo.git.add_all()?;
        let status = repo.git.status()?;

        if repo.lfs {
            for change in &status {
                _ = twinkle_lfs_track(repo, change);
            }
        }

        if let Some(message) = twinkle_commit_message(&status) {
            repo.git.config_set_user(&repo.user)?;
            repo.git.commit(&repo.user, &message)?;

            log::info(&format!("✓ Committed. Now at {}", repo.current_head()?));
        } else {
            log::info(&format!("Nothing new to commit. Still at {}", repo.current_head()?));
        }

        repo.git.lfs_install_pre_push_hook()?;
        let push = repo.git.push("origin", &repo.branch);

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

        if !twinkle_has_unpushed_commits(repo) {
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

    if repo.lfs {
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


pub enum TwinkleStatus {
    UpToDate(i64), //(last check)
    Error(String),
    UnpushedChanges,
}


pub enum TwinklePushError {
    NoNetwork(String),
    NoAuth(String),
    RemoteAhead(String),
    Unknown(String),
}

pub enum TwinkleFetchError {
    NoNetwork(String),
    NoAuth(String),
    NoRepository(String),
    Unknown(String),
}
