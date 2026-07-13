//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::{ Path, PathBuf };
use std::sync::{ Arc, Mutex };

use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::reference::GitReference;


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


// Convenience
impl TwinkleRepository {
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


// Sync
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
