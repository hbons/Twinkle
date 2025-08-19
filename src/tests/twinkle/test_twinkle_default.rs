//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::ssh::objects::url::SshUrl;

use crate::twinkle::twinkle_default::twinkle_default_attribute_rules;
use crate::twinkle::twinkle_default::twinkle_default_exclude_rules;
use crate::twinkle::twinkle_default::twinkle_default_file_warning;
use crate::twinkle::twinkle_default::twinkle_default_settings;
use crate::twinkle::twinkle_default::twinkle_default_welcome;


#[test]
fn test_twinkle_default_settings() {
    let settings = twinkle_default_settings();
    assert_eq!(settings.len(), 18);
}


#[test]
fn test_twinkle_default_exclude_rules() {
    let rules = twinkle_default_exclude_rules();
    assert_eq!(rules.len(), 16);
}


#[test]
fn test_twinkle_default_attribute_rules() {
    let rules = twinkle_default_attribute_rules();
    assert_eq!(rules.len(), 2);
}


#[test]
fn test_twinkle_default_welcome() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let message = twinkle_default_welcome(&url);

    assert!(message.contains(url.original.as_str()));
    assert!(message.len() > url.original.len());
}


#[test]
fn test_twinkle_default_file_warning() {
    let warning = twinkle_default_file_warning();
    assert!(warning.starts_with("#"));
}
