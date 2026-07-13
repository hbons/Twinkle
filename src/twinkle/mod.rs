//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub mod defaults {
    pub mod common;
    pub mod config;
    pub mod hosts;
    pub mod info;
}

pub mod objects {
    pub mod repository;
    pub mod repository_files;
    pub mod repository_config;
    pub mod repository_notify;
}

pub mod twinkle_clone;
pub mod twinkle_init;
pub mod twinkle_keys;
pub mod twinkle_lfs;
pub mod twinkle_notify;
pub mod twinkle_pretty;
pub mod twinkle_resolve;
pub mod twinkle_util;
pub mod twinkle_sync;
