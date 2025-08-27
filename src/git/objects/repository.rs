//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

// TODO: TwinkleRepository. Move to twinkle crate?
// this way we can use the twinkle_default_* functions


use std::error::Error;
use std::fs::{ self, File };
use std::io::Write;
use std::path::{ Path, PathBuf };
use std::sync::{ Arc, Mutex };

use serde::{ Serialize, Deserialize };

use crate::log;
use crate::ssh::objects::url::SshUrl;

use super::environment::GitEnvironment;
use super::user::GitUser;


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct GitRepository {
    pub path: PathBuf,
    pub remote_url: SshUrl,
    pub branch: String,

    pub large_file_storage: bool,
    pub lfs_threshold: Option<u64>,
    pub polling_interval: Option<u64>,

    pub user: GitUser,

    #[serde(skip)] pub last_checked: i64,
    #[serde(skip)] pub last_synced:  i64,

    #[serde(skip)] pub git: GitEnvironment,
    #[serde(skip)] is_syncing: Arc<Mutex<bool>>,
    #[serde(skip)] has_local_changes:  Arc<Mutex<bool>>,
    #[serde(skip)] has_remote_changes: Arc<Mutex<bool>>,
}


impl GitRepository {
    pub fn new(path: PathBuf, remote_url: SshUrl, branch: String) -> GitRepository {
        GitRepository {
            path,
            remote_url,
            branch,
            ..Default::default()
        }
    }
}


impl Default for GitRepository {
    fn default() -> Self {
        GitRepository {
            path: PathBuf::default(),
            remote_url: SshUrl::default(),
            branch: "main".to_string(),

            large_file_storage: false,
            lfs_threshold: None,

            user: GitUser::default(),

            // is_paused: false,
            polling_interval: None,

            last_checked: 0,
            last_synced:  0,

            git: GitEnvironment::default(),
            is_syncing: Arc::new(Mutex::new(false)),
            has_local_changes:  Arc::new(Mutex::new(false)),
            has_remote_changes: Arc::new(Mutex::new(false)),
        }
    }
}


impl GitRepository {
    /// Current long commit hash
    pub fn current_head(&self) -> Result<String, Box<dyn Error>> {
        self.git.rev_parse()
    }


    /// Absolute path to the path in the repository
    pub fn path(&self, path: &Path) -> PathBuf {
        self.path.join(path)
    }
}


impl GitRepository { // TwinkleRepository
    pub fn write_config(&self) -> Result<(), Box<dyn Error>> {
        self.git.config_set("twinkle.lastChecked", &self.last_checked.to_string())?;
        self.git.config_set("twinkle.lastSynced", &self.last_synced.to_string())?;
        Ok(())
    }
}


impl GitRepository {
    pub fn write_attribute_rules(&self, rules: Vec<String>) -> Result<(), Box<dyn Error>> {
        let attributes_path = self.git.working_dir.join(".git/info/attributes");
        let mut buffer = File::create(&attributes_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", &attributes_path.to_string_lossy()));
        Ok(())
    }


    pub fn write_exclude_rules(&self, rules: Vec<String>) -> Result<(), Box<dyn Error>> {
        let exclude_path = self.git.working_dir.join(".git/info/exclude");
        let mut buffer = File::create(&exclude_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", &exclude_path.to_string_lossy()));
        Ok(())
    }
}


impl GitRepository { // TwinkleRepository
    pub fn set_has_local_changes(&self, value: bool) {
        if let Ok(mut v) = self.has_local_changes.lock() {
            *v = value;
        }
    }

    pub fn set_has_remote_changes(&self, value: bool) {
        if let Ok(mut v) = self.has_remote_changes.lock() {
            *v = value;
        }
    }

    pub fn has_local_changes(&self) -> bool {
        *self.has_local_changes.lock().unwrap()
    }

    pub fn has_remote_changes(&self) -> bool {
        *self.has_remote_changes.lock().unwrap()
    }
}


impl GitRepository { // TwinkleRepository
    pub fn set_is_syncing(&self, value: bool) {
        if let Ok(mut v) = self.is_syncing.lock() {
            *v = value;
        }
    }

    pub fn is_syncing(&self) -> bool {
        *self.is_syncing.lock().unwrap()
    }
}


impl GitRepository {
    pub fn size_of(&self, path: &Path) -> Option<u64> {
        let path = self.path.join(path);
        let metadata = fs::metadata(path).ok()?;

        Some(metadata.len())
    }
}
