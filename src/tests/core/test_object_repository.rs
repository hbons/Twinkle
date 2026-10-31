//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::time::Duration;

use crate::core::objects::repository_options_sync::parse_lfs_size;
use crate::core::objects::repository_options_sync::parse_polling_interval;


#[test]
fn test_object_repository_parse_lfs_size() {
    assert_eq!(parse_lfs_size(""), 0);
    assert_eq!(parse_lfs_size("0"), 0);
    assert_eq!(parse_lfs_size("1"), 1);
    assert_eq!(parse_lfs_size("1k"), 1024);
    assert_eq!(parse_lfs_size("1K"), 1024);
    assert_eq!(parse_lfs_size("-1k"), 0);
    assert_eq!(parse_lfs_size("k"), 0);
    assert_eq!(parse_lfs_size("K"), 0);
    assert_eq!(parse_lfs_size("2k"), 2048);
    assert_eq!(parse_lfs_size("1m"), 1024 * 1024);
    assert_eq!(parse_lfs_size("2m"), 1024 * 1024 * 2);
    assert_eq!(parse_lfs_size("1g"), 1024 * 1024 * 1024);
}


#[test]
fn test_object_repository_parse_polling_interval() {
    assert_eq!(parse_polling_interval("-1"), Duration::from_secs(0));
    assert_eq!(parse_polling_interval(""), Duration::from_secs(0));
    assert_eq!(parse_polling_interval("0"), Duration::from_secs(0));
    assert_eq!(parse_polling_interval("1"), Duration::from_secs(1));
    assert_eq!(parse_polling_interval("1s"), Duration::from_secs(1));
    assert_eq!(parse_polling_interval("1S"), Duration::from_secs(1));
    assert_eq!(parse_polling_interval("-1s"), Duration::from_secs(0));
    assert_eq!(parse_polling_interval("s"), Duration::from_secs(0));
    assert_eq!(parse_polling_interval("S"), Duration::from_secs(0));
    assert_eq!(parse_polling_interval("2s"), Duration::from_secs(2));
    assert_eq!(parse_polling_interval("1m"), Duration::from_secs(60));
    assert_eq!(parse_polling_interval("2m"), Duration::from_secs(60 * 2));
    assert_eq!(parse_polling_interval("1h"), Duration::from_secs(60 * 60));
}
