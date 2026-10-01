//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::ssh::objects::url::SshUrl;

use crate::core::defaults::info::default_info_attributes;
use crate::core::defaults::info::default_info_exclude;
use crate::core::defaults::config::default_git_settings;
use crate::core::init::init_welcome;


#[test]
fn test_default_settings() {
    let settings = default_git_settings();
    assert!(!settings.is_empty());
}


#[test]
fn test_default_exclude_rules() {
    let rules = default_info_exclude();
    assert!(!rules.is_empty());
}


#[test]
fn test_default_attribute_rules() {
    let rules = default_info_attributes();
    assert!(!rules.is_empty());
}


#[test]
fn test_default_welcome() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let message = init_welcome(&url);

    assert!(message.contains(url.original.as_str()));
    assert!(message.len() > url.original.len());
}
