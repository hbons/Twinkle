//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fs;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use crate::ssh::objects::url::SshUrl;

use crate::core::defaults::config::twinkle_default_git_settings;
use crate::core::defaults::info::twinkle_default_info_attributes;
use crate::core::defaults::info::twinkle_default_info_exclude;
use crate::core::init::init_repo;


#[test]
fn test_twinkle_init() {
    let tmp_path = Path::new("/tmp/twinkle_tests");

    let path = tmp_path;
    let remote_url = "git@github.com:hbons/SparkleShare"
        .parse::<SshUrl>().unwrap();

    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }

    fs::create_dir_all(path).unwrap();

    let result = init_repo(path, &remote_url, None);

    assert!(result.is_ok());


    let git = GitEnvironment::new(path);

    assert_eq!(git.config_get("twinkle.enabled").unwrap(), "true");
    assert!(git.config_get("twinkle.id").is_some());
    assert_eq!(git.config_get("remote.origin.url").unwrap(), remote_url.to_string_standard());

    for rule in twinkle_default_git_settings() {
        assert_eq!(git.config_get(rule.0).unwrap(), rule.1);
    }

    let branch = git.branch_show_current().unwrap();
    assert_eq!(branch, "main");


    let path = tmp_path.join(".git/info/attributes");

    assert_eq!(
        twinkle_default_info_attributes().len(),
        fs::read_to_string(&path).unwrap().lines().count(),
    );


    let path = tmp_path.join(".git/info/exclude");

    assert_eq!(
        twinkle_default_info_exclude().len(),
        fs::read_to_string(&path).unwrap().lines().count(),
    );


    let path = tmp_path.join(".twinkle/config");
    assert!(path.exists());

    let path = tmp_path.join("TWINKLE.md");
    assert!(path.exists());

    assert_eq!(git.log(2).unwrap().len(), 1);
    assert_eq!(git.log(1).unwrap().first().unwrap().changes.len(), 2);
}
