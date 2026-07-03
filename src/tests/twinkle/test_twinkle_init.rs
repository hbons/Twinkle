//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fs;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use crate::ssh::objects::url::SshUrl;

use crate::twinkle::defaults::config::twinkle_default_git_settings;
use crate::twinkle::defaults::info::twinkle_default_info_attributes;
use crate::twinkle::defaults::info::twinkle_default_info_exclude;
use crate::twinkle::twinkle_init::twinkle_init;


#[test]
fn test_twinkle_init() {
    let path = Path::new("/tmp/twinkle_tests");
    let remote_url = "git@github.com:hbons/SparkleShare"
        .parse::<SshUrl>().unwrap();

    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }

    fs::create_dir_all(path).unwrap();

    let result = twinkle_init(path, &remote_url, None);

    assert!(result.is_ok());


    let git = GitEnvironment::new(path);

    assert_eq!(git.config_get("twinkle.enabled").unwrap().stdout, "true");
    assert!(git.config_get("twinkle.id").is_ok());
    assert_eq!(git.config_get("remote.origin.url").unwrap().stdout, remote_url.to_string_standard());

    for rule in twinkle_default_git_settings() {
        assert_eq!(git.config_get(rule.0).unwrap().stdout, rule.1);
    }


    let path = Path::new("/tmp/twinkle_tests/.git/info/attributes");

    assert_eq!(
        twinkle_default_info_attributes().len(),
        fs::read_to_string(&path).unwrap().lines().count(),
    );


    let path = Path::new("/tmp/twinkle_tests/.git/info/exclude");

    assert_eq!(
        twinkle_default_info_exclude().len(),
        fs::read_to_string(&path).unwrap().lines().count(),
    );


    let path = Path::new("/tmp/twinkle_tests/.twinkle.conf");
    assert!(path.exists());

    let path = Path::new("/tmp/twinkle_tests/TWINKLE.md");
    assert!(path.exists());

    assert_eq!(git.log(2).unwrap().len(), 1);
    assert_eq!(git.log(1).unwrap().first().unwrap().changes.len(), 2);
}
