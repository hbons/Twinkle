//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs::{ self, File };
use std::io::Write;
use std::path::{ Path, PathBuf };
use std::sync::{ Arc, Mutex };
use std::time::Duration;

use sha2::{ Sha256, Digest };

use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::reference::GitReference;
use crate::git::objects::user::GitUser;
use crate::log;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::objects::config::SshConfig;
use crate::ssh::objects::url::SshUrl;
use crate::twinkle::defaults::common::twinkle_default_polling_interval;
use crate::twinkle::defaults::config::{
    K_ENABLED,
    K_ID,
    K_LAST_CHECK,
    K_LAST_SYNC,
    K_LFS_ENABLED,
    K_LFS_SIZE_THRESHOLD,
    K_POLLING_INTERVAL,
    K_READONLY,
    key
};

use crate::twinkle::twinkle_lfs::TWINKLE_LFS_THRESHOLD;





#[derive(Clone, Debug, Default)]
pub struct TwinkleRepository {
    pub path: PathBuf,
    pub git: GitEnvironment,

    is_busy: Arc<Mutex<bool>>,
    has_local_changes:  Arc<Mutex<bool>>,
    has_remote_changes: Arc<Mutex<bool>>,
}


impl TwinkleRepository {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            git: GitEnvironment::new(path),
            ..Default::default()
        }
    }
}


// enabled
impl TwinkleRepository {
    pub fn enabled(&self) -> bool {
        if let Ok(output) = self.git.config_get(&key(K_ENABLED)) {
            if let Ok(value) = output.stdout.parse::<bool>() {
                return value;
            }
        }

        false
    }

    pub fn set_enabled(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_ENABLED),
            &value.to_string()
        )?;

        Ok(())
    }
}


// id
impl TwinkleRepository {
    /// Random SHA-256 stored in .git/config
    pub fn id(&self) -> Option<String> {
        self.git.config_get(&key(K_ID)).ok()
            .map(|r| r.stdout)
    }

    pub fn set_id(&self, value: &String) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_ID),
            value
        )?;

        Ok(())
    }
}


// read_only
impl TwinkleRepository {
    pub fn read_only(&self) -> bool {
        if let Ok(output) = self.git.config_get(&key(K_READONLY)) {
            if let Ok(value) = output.stdout.parse::<bool>() {
                return value;
            }
        }

        false
    }

    pub fn set_read_only(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_READONLY),
            &value.to_string()
        )?;

        Ok(())
    }
}


// remote_url
impl TwinkleRepository {
    pub fn remote_url(&self) -> Option<SshUrl> {
        self.git.config_get("remote.origin.url")
            .ok()
            .and_then(|v|
                v.stdout.trim().parse::<SshUrl>().ok()
            )
    }

    pub fn set_remote_url(&self, value: &SshUrl) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            "remote.origin.url",
            &value.to_string_standard()
        )?;

        Ok(())
    }
}


// user
impl TwinkleRepository {
    pub fn user(&self) -> Option<GitUser> {
        GitUser::from(&self.path).ok()
    }


    pub fn set_user(&self, value: &GitUser) -> Result<(), Box<dyn Error>>{
        self.git.config_set("user.name", value.name())?;
        self.git.config_set("user.email", value.email())?;

        if let Some(key_pair) = &value.key_pair {
            self.set_user_signing_key(key_pair)?;
        }

        Ok(())
    }
}


// polling_interval
impl TwinkleRepository {
    pub fn polling_interval(&self) -> Duration {
        let default = twinkle_default_polling_interval().as_secs();

        match self.git.config_get(&key(K_POLLING_INTERVAL)) {
            Ok(output) => Duration::from_mins(parse_polling_interval(&output.stdout)),
            Err(_) => Duration::from_secs(default),
        }
    }

    pub fn set_polling_interval(&self, value: Duration) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_POLLING_INTERVAL),
            &value.as_secs().to_string(),
        )?;

        Ok(())
    }
}

pub fn parse_polling_interval(s: &str) -> u64 {
    let (number, multiplier) = match s.as_bytes().last() {
        Some(b's' | b'S') => (&s[..s.len() - 1], 1),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 60),
        Some(b'h' | b'H') => (&s[..s.len() - 1], 60 * 60),
        _ => (s, 1),
    };

    number.parse::<u64>().unwrap_or(0) * multiplier
}


// last_synced, last_checked
impl TwinkleRepository {
    pub fn last_checked(&self) -> Option<i64> {
        self.git.config_get(&key(K_LAST_CHECK))
            .ok()
            .and_then(|v|
                v.stdout.parse::<i64>().ok()
            )
    }

    pub fn set_last_checked(&self, value: i64) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_LAST_CHECK),
            &value.to_string(),
        )?;

        Ok(())
    }


    pub fn last_synced(&self) -> Option<i64> {
        self.git.config_get(&key(K_LAST_SYNC))
            .ok()
            .and_then(|v|
                v.stdout.parse::<i64>().ok()
            )
    }

    pub fn set_last_synced(&self, value: i64) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_LAST_SYNC),
            &value.to_string(),
        )?;

        Ok(())
    }
}


// lfs
impl TwinkleRepository {
    pub fn lfs_enabled(&self) -> bool {
        if let Ok(output) = self.git.config_get(&key(K_LFS_ENABLED)) {
            if let Ok(value) = output.stdout.parse::<bool>() {
                return value;
            }
        }

        false
    }

    pub fn set_lfs_enabled(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_LFS_ENABLED),
            &value.to_string(),
        )?;

        Ok(())
    }


    pub fn lfs_size_threshold(&self) -> u64 {
        let default = TWINKLE_LFS_THRESHOLD;

        match self.git.config_get(&key(K_LFS_SIZE_THRESHOLD)) {
            Ok(output) => parse_lfs_size(&output.stdout),
            Err(_) => default,
        }
    }

    pub fn set_lfs_size_threshold(&self, value: u64) -> Result<(), Box<dyn Error>>{
        self.git.config_set(
            &key(K_LFS_SIZE_THRESHOLD),
            &value.to_string(),
        )?;

        Ok(())
    }
}

pub fn parse_lfs_size(s: &str) -> u64 {
    let (number, multiplier) = match s.as_bytes().last() {
        Some(b'k' | b'K') => (&s[..s.len() - 1], 1024),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 1024 * 1024),
        Some(b'g' | b'G') => (&s[..s.len() - 1], 1024 * 1024 * 1024),
        _ => (s, 1),
    };

    number.parse::<u64>().unwrap_or(0) * multiplier
}


pub type TwinkleChannelName = String;

// Convenience
impl TwinkleRepository {
    /// Channel name for notifications
    /// SHA-256 of the repository id
    pub fn as_notify_channel_name(&self) -> Option<TwinkleChannelName> {
        self.id().map(|s|
            Sha256::digest(s)
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect()
        )
    }


    /// Current branch
    pub fn branch(&self) -> Option<GitReference> {
        if self.git.symbolic_ref().is_err() {
            return None; // detached HEAD
        }

        self.git.branch_show_current().ok()
    }


    /// Current long commit hash
    pub fn current_head(&self) -> Result<GitReference, Box<dyn Error>> {
        self.git.rev_parse()
    }


    /// Absolute path to the path in the repository
    pub fn abs_path(&self, path: &Path) -> PathBuf {
        self.path.join(path)
    }


    pub fn size_of(&self, path: &Path) -> Option<u64> {
        let path = self.path.join(path);
        fs::metadata(path)
            .ok()
            .map(|v| v.len())
    }


    pub fn is_empty(&self) -> bool {
        self.git.rev_parse().is_err()
    }
}


impl TwinkleRepository {
    pub fn write_attribute_rules(&self, rules: Vec<String>) -> Result<(), Box<dyn Error>> {
        let attributes_path = self.git.working_dir.join(".git/info/attributes");
        let mut buffer = File::create(&attributes_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", &attributes_path.to_string_lossy()));

        Ok(())
    }


    pub fn write_exclude_rules(&self, rules: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let exclude_path = self.git.working_dir.join(".git/info/exclude");
        let mut buffer = File::create(&exclude_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", &exclude_path.to_string_lossy()));

        Ok(())
    }
}


impl TwinkleRepository {
    pub fn is_busy(&self) -> bool {
        *self.is_busy.lock().unwrap()
    }

    pub fn set_is_busy(&self, value: bool) {
        if let Ok(mut v) = self.is_busy.lock() {
            *v = value;
        }
    }


    pub fn has_local_changes(&self) -> bool {
        *self.has_local_changes.lock().unwrap()
    }

    pub fn set_has_local_changes(&self, value: bool) {
        if let Ok(mut v) = self.has_local_changes.lock() {
            *v = value;
        }
    }


    pub fn has_remote_changes(&self) -> bool {
        *self.has_remote_changes.lock().unwrap()
    }

    pub fn set_has_remote_changes(&self, value: bool) {
        if let Ok(mut v) = self.has_remote_changes.lock() {
            *v = value;
        }
    }
}


impl TwinkleRepository {
    pub fn set_user_signing_key(&self, key_pair: &KeyPair) -> Result<(), Box<dyn Error>>{
        let key_path = &key_pair.private_key_path.to_string_lossy();
        self.git.config_set("user.signingKey", key_path)?;

        Ok(())
    }


    pub fn set_commit_gpg_sign(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.git.config_set("commit.gpgSign", &value.to_string())?;
        Ok(())
    }

    pub fn set_tag_gpg_sign(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.git.config_set("tag.gpgSign", &value.to_string())?;
        Ok(())
    }


    // Write a minimal SSH command to the .git/config for debugging purposes
    pub fn set_core_ssh_command(&self, key_pair: Option<&KeyPair>) -> Result<(), Box<dyn Error>>{
        let config = match key_pair {
            Some(key_pair) => {
                SshConfig {
                    IdentitiesOnly: true,
                    IdentityFile: Some(key_pair.private_key_path.clone()),
                    UserKnownHostsFile: Some(key_pair.private_key_path.with_extension("key.host")),
                    ..Default::default()
                }
            },
            None => SshConfig::default(),
        };

        self.git.config_set("core.sshCommand", // TODO: consts in git crate
            &format!("ssh -F /dev/null {config}"))?;

        Ok(())
    }
}
