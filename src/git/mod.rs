//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


//  This is not meant to be a full implementation of Git, just the
//  parts needed by Twinkle and an opportunity for me to learn Rust.
//
//  – Hylke

pub mod objects {
    pub mod change;
    pub mod commit;
    pub mod commit_message;
    pub mod environment;
    pub mod error;
    pub mod file_status;
    pub mod merge_status;
    pub mod repository;
    pub mod output;
    pub mod user;
}

pub mod add;
pub mod branch;
pub mod checkout;
pub mod clone;
pub mod commit;
pub mod config;
pub mod fetch;
pub mod lfs;
pub mod log;
pub mod ls_files;
pub mod ls_remote;
pub mod merge;
pub mod push;
pub mod rev_parse;
pub mod rev_list;
pub mod status;
pub mod version;
