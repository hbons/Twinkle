//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;

use chrono::Utc;

use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::user::GitUser;

use crate::ssh::keygen::ssh_keygen_fingerprint;
use crate::ssh::keys::host_key::HostKey;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::objects::url::SshUrl;
use crate::ssh::util::ssh_util_test_connection;

use super::objects::twinkle_repository::TwinkleRepository;
use super::twinkle_default::twinkle_default_branch;
use super::twinkle_default::twinkle_default_commit;
use super::twinkle_default::twinkle_default_init;
use super::twinkle_keys::twinkle_hostkey_for;
use super::twinkle_keys::twinkle_keypair_for;
use super::twinkle_util::twinkle_default_dir_name;
use super::twinkle_util::twinkle_ssh_command;
use super::twinkle_util::twinkle_unique_dir;


pub fn twinkle_clone_prepare(url: &SshUrl, keys_dir: &Path) -> Result<KeyPair, Box<dyn Error>> {
    let key_pair = twinkle_keypair_for(&url.host, KeyType::default(), keys_dir)?;

    let mut host_key = match twinkle_hostkey_for(url, KeyType::default(), keys_dir) {
        Err(_) => { return Err(Box::new(TwinkleCloneError::NeedsNetwork)); }
        Ok(host_key) => host_key,
    };

    if host_key.fingerprint.is_none() {
        let fingerprint = ssh_keygen_fingerprint(&host_key)?;
        host_key.fingerprint = Some(fingerprint);
    }

    if !host_key.is_trusted {
        Err(Box::new(TwinkleCloneError::NeedsTrust(host_key)))
    } else {
        match ssh_util_test_connection(url, &host_key, &key_pair) {
            Err(_) => Err(Box::new(TwinkleCloneError::NeedsAuth(host_key, key_pair))),
            Ok(_) => Ok(key_pair),
        }
    }
}


pub fn twinkle_clone_start(url: &SshUrl, key_pair: &KeyPair, path: &Path) -> Result<TwinkleRepository, Box<dyn Error>> {
    let git = GitEnvironment {
        working_dir: path.to_path_buf(),
        GIT_SSH_COMMAND: twinkle_ssh_command(key_pair),
        ..Default::default()
    };

    let dir = twinkle_default_dir_name(url)?;
    let dir = twinkle_unique_dir(&dir);
    let target_git = git.clone(&url.to_string_standard(), Some(dir.as_ref()), Some(1))?;

    let branch = twinkle_default_branch();
    let mut repo = TwinkleRepository::new(target_git.working_dir.clone(), url.clone(), branch.into());
    repo.git = target_git;

    if !repo.git.lfs_ls_files()?.is_empty() {
        repo.lfs = true;
        repo.git.lfs_fetch()?;
    }

    Ok(repo)
}


pub fn twinkle_clone_complete(repo: &mut TwinkleRepository, key_pair: &KeyPair) -> Result<(), Box<dyn Error>> {
    twinkle_default_init(repo)?;

    repo.user = GitUser {
        key_pair: Some(key_pair.clone()),
        ..Default::default()
    };

    repo.git.config_set_user(&repo.user)?;
    repo.git.config_set_user_signing_key(&key_pair)?;

    repo.git.config_set_core_ssh_command(&key_pair)?;
    repo.git.checkout_branch("HEAD")?;

    if repo.git.is_repo_empty() {
        twinkle_default_commit(repo)?;
    }

    repo.branch = repo.git.branch_show_current()?;
    repo.last_checked = Utc::now().timestamp();
    repo.last_synced  = Utc::now().timestamp();

    Ok(())
}


#[derive(Debug)]
pub enum TwinkleCloneError {
    NeedsNetwork,
    NeedsTrust(HostKey),
    NeedsAuth(HostKey, KeyPair),
}

impl fmt::Display for TwinkleCloneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TwinkleCloneError::NeedsNetwork => write!(f, "No network connection"),
            TwinkleCloneError::NeedsTrust(host_key) => write!(f, "Host key not trusted: {}", host_key),
            TwinkleCloneError::NeedsAuth(_host_key, _key_pair) => write!(f, "Authentication failed with host key and key pair"),
        }
    }
}

impl Error for TwinkleCloneError {}
