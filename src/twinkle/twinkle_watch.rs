//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::consts::OS;
use std::error::Error;
use std::thread;
use std::time::Duration;

use chrono::Utc;

use crate::git::objects::repository::GitRepository;
use crate::log;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::util::ssh_util_test_connection;
use crate::twinkle::twinkle_default::{ twinkle_default_init, twinkle_default_polling_interval };
use crate::twinkle::twinkle_lfs::twinkle_lfs_track;

use super::twinkle_keys::twinkle_hostkey_for;
use super::twinkle_resolve::twinkle_resolve_changes;
use super::twinkle_util::{ twinkle_commit_message, twinkle_ssh_command };


pub fn twinkle_watch(repo: &mut GitRepository, key_pair: &KeyPair) -> Result<(), Box<dyn Error>> {
    if repo.git.branch_show_current()? != repo.branch {
        return Err(format!("Repository not on branch as set in config ({})", repo.branch).into())
    }

    let host_key = twinkle_hostkey_for(&repo.remote_url, KeyType::default(),
        &key_pair.private_key_path.parent().ok_or("No parent")?)?;

    repo.git.GIT_SSH_COMMAND = twinkle_ssh_command(&key_pair);

    twinkle_default_init(repo)?;
    repo.git.config_set_user(&repo.user)?;
    repo.git.config_set_user_signing_key(&key_pair)?;

    ssh_util_test_connection(&repo.remote_url, &host_key, &key_pair)?;
    log::debug(&format!("{} | Authenticated", &repo.remote_url.host));

    let repo_c1 = repo.clone();
    let mut repo_c2 = repo.clone();
    thread::spawn(move || { _ = twinkle_watch_local(&repo_c1); });
    thread::spawn(move || { _ = twinkle_watch_remote(&mut repo_c2); });

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


pub fn twinkle_watch_local(repo: &GitRepository) -> Result<(), Box<dyn Error>> {
    loop {
        if !repo.is_syncing() {
            let status = repo.git.status()?;
            if !status.is_empty() { // TODO: Manual status every 5min or so once we have a FS watcher
                repo.set_has_local_changes(true);
                log::info("Local changes detected…");
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}


pub fn twinkle_watch_remote(repo: &mut GitRepository) -> Result<(), Box<dyn Error>> {
    loop {
        let interval = repo.polling_interval
            .unwrap_or(twinkle_default_polling_interval());

        let local_id = repo.current_head()?;

        if !repo.is_syncing() {
            if let Ok(remote_id) = repo.git.ls_remote(&repo.branch) {
                if remote_id != local_id {
                    repo.set_has_remote_changes(true);
                    log::info("Remote changes detected…");
                }
            }

            repo.last_checked = Utc::now().timestamp();
        }

        thread::sleep(Duration::from_secs(interval));
    }
}


pub fn twinkle_sync_up(repo: &mut GitRepository) -> Result<(), Box<dyn Error>> {
    // TODO: Needs exponential backoff on error here

    loop {
        thread::sleep(Duration::from_secs(1));

        repo.git.lfs_config_filters()?;
        repo.git.add_all()?;
        let status = repo.git.status()?;

        if repo.large_file_storage {
            for change in &status {
                _ = twinkle_lfs_track(repo, change);
            }
        }

        if let Some(message) = twinkle_commit_message(&status) {
            repo.git.config_set_user(&repo.user)?;
            repo.git.commit(&repo.user, &message)?;

            log::info(&format!("Committed. Now at {}", repo.current_head()?));
        } else {
            log::info(&format!("Nothing new to commit. Still at {}", repo.current_head()?));
        }

        repo.git.lfs_install_pre_push_hook()?;

        match repo.git.push("origin", &repo.branch) {
            Ok(_)  => log::info(&format!("Pushed. Local and remote at {}", repo.current_head()?)),
            Err(_) => {
                log::info("Push failed (remote is ahead). Fetching…");
                twinkle_sync_down(repo)?;
            }
        }

        if !twinkle_has_unpushed_commits(repo) {
            break;
        }
    }

    Ok(())
}


pub fn twinkle_sync_down(repo: &mut GitRepository) -> Result<(), Box<dyn Error>> {
    repo.git.fetch("main")?;
    repo.git.lfs_fetch()?;

    if OS == "macos" { repo.git.config_set("core.ignoreCase", "true")?; }
    let merge = repo.git.merge("FETCH_HEAD");

    if merge.is_err() {
        twinkle_resolve_changes(repo)?;
    }

    if OS == "macos" { repo.git.config_set("core.ignoreCase", "false")?; }

    log::info(&format!("Fetched and merged. Now at {}", repo.current_head()?));
    Ok(())
}


pub fn twinkle_has_unpushed_commits(repo: &GitRepository) -> bool {
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
