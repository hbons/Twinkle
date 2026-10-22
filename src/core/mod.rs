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
    pub mod channel;
    pub mod push_connection;
    pub mod repository;
    pub mod repository_files;
    pub mod repository_config;
}

pub mod clone;
pub mod init;
pub mod keys;
pub mod lfs;
pub mod push;
pub mod pretty;
pub mod resolve;
pub mod sync;
pub mod watch_local;
pub mod watch_remote;
pub mod util;
