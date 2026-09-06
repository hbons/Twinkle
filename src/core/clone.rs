//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::fmt;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;

use crate::core::init;
use crate::log;

use crate::ssh;
use crate::ssh::keys::host_key::HostKey;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::objects::url::SshUrl;

use super::objects::repository::TwinkleRepository;
use super::keys;
use super::util;


pub fn prepare_keys(
    url: &SshUrl,
    keys_dir: &Path,
) -> Result<KeyPair, Box<dyn Error>>
{
    let key_pair = keys::keypair_for(
        &url.host,
        KeyType::default(),
        keys_dir
    )?;

    let mut host_key = match keys::hostkey_for(url, KeyType::default(), keys_dir) {
        Err(_) => { return Err(Box::new(TwinkleCloneError::NeedsNetwork)); }
        Ok(host_key) => host_key,
    };

    if host_key.fingerprint.is_none() {
        let fingerprint = ssh::keygen::derive_fingerprint(&host_key)?;
        host_key.fingerprint = Some(fingerprint);
    }

    if !host_key.is_trusted {
        Err(Box::new(TwinkleCloneError::NeedsTrust(host_key)))
    } else {
        match ssh::util::test_connection(url, &host_key, Some(&key_pair)) {
            Err(_) => Err(Box::new(TwinkleCloneError::NeedsAuth(host_key, key_pair))),
            Ok(_) => Ok(key_pair),
        }
    }
}


pub fn start(
    url: &SshUrl,
    key_pair: Option<&KeyPair>,
    path: Option<&Path>,
) -> Result<TwinkleRepository, Box<dyn Error>>
{
    let git = GitEnvironment {
        working_dir: env::current_dir()?,
        GIT_SSH_COMMAND: util::ssh_command(key_pair),
        ..Default::default()
    };

    if git.rev_parse_show_toplevel().is_ok() {
        return Err("Already inside a Git repository".into());
    }

    let dir = if let Some(p) = path {
        util::unique_dir(p)
    } else {
        let d = url.path.file_stem().ok_or("Could not determine path")?;
        util::unique_dir(Path::new(&d))
    };

    let target_git = git.clone(url, &dir)?;

    let mut repo = TwinkleRepository::new(&target_git.working_dir)?;
    repo.git = target_git;

    if let Ok(lfs_files) = repo.git.lfs_ls_files() {
        if !lfs_files.is_empty() {
            log::info("LFS files detected. Fetching…");
            repo.set_lfs_enabled(true)?;
            repo.git.lfs_fetch()?;
        }
    }

    Ok(repo)
}


pub fn complete(
    repo: &mut TwinkleRepository,
    key_pair: Option<&KeyPair>,
) -> Result<(), Box<dyn Error>>
{
    init::init_common(repo, key_pair)?;

    if repo.is_empty() {
        init::init_id(repo)?;
        init::init_first_commit(repo)?;
    } else {
        repo.git.checkout_branch(&"HEAD".into())?;
        init::init_id(repo)?;
    }

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
