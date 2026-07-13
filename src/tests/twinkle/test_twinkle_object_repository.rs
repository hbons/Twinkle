//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::objects::repository_config::parse_lfs_size;
use crate::twinkle::objects::repository_config::parse_polling_interval;


#[test]
fn test_twinkle_object_repository_parse_lfs_size() {
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
fn test_twinkle_object_repository_parse_polling_interval() {
    assert_eq!(parse_polling_interval("-1"), 0);
    assert_eq!(parse_polling_interval(""), 0);
    assert_eq!(parse_polling_interval("0"), 0);
    assert_eq!(parse_polling_interval("1"), 1);
    assert_eq!(parse_polling_interval("1s"), 1);
    assert_eq!(parse_polling_interval("1S"), 1);
    assert_eq!(parse_polling_interval("-1s"), 0);
    assert_eq!(parse_polling_interval("s"), 0);
    assert_eq!(parse_polling_interval("S"), 0);
    assert_eq!(parse_polling_interval("2s"), 2);
    assert_eq!(parse_polling_interval("1m"), 60);
    assert_eq!(parse_polling_interval("2m"), 60 * 2);
    assert_eq!(parse_polling_interval("1h"), 60 * 60);
}
