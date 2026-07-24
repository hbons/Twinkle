//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


#[allow(clippy::module_inception)]
pub mod check {
    pub mod check;
    pub mod check_config;
    pub mod check_platform;
    pub mod check_repository;
    pub mod check_ssh;
}

pub mod args;
pub mod clone;
pub mod init;
pub mod sync;
pub mod status;
pub mod util;
