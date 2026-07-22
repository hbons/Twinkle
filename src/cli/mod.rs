//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub mod checklist {
    pub mod checklist;
    pub mod checklist_config;
    pub mod checklist_platform;
    pub mod checklist_repository;
    pub mod checklist_ssh;
}

pub mod args;
pub mod clone;
pub mod init;
pub mod sync;
pub mod status;
pub mod util;
