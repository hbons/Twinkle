//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::{ Path, PathBuf };
use std::str::FromStr;

use crate::twinkle::objects::twinkle_repository::TwinkleRepository;
use crate::twinkle::twinkle_config::TwinkleConfig;


pub const CONFIG_JSON: &str = r#"
[
  {
    "path": "/Users/hbons/Projects/twinkle/TwinkleTest",
    "remote_url": "ssh://git@github.com/hbons/TwinkleTest",
    "branch": "main",
    "lfs": false,
    "lfs_threshold": null,
    "polling_interval": null,
    "user": {
      "name": "Unknown",
      "email": "git@localhost"
    }
  }
]
"#;


#[test]
fn test_twinkle_config_new() {
    let path = Path::new("/tmp");
    let config = TwinkleConfig::new(path);

    assert_eq!(config.config_path(), Some(path.to_path_buf()));
    assert_eq!(config.list().unwrap().len(), 0);
}


#[test]
fn test_twinkle_config_from_str() {
    let json = CONFIG_JSON;
    let result = TwinkleConfig::from_str(json);

    assert!(result.is_ok());

    let mut json = CONFIG_JSON.to_string();
    json.push_str("}");
    let result = TwinkleConfig::from_str(json.as_str());

    assert!(result.is_err());
}


#[test]
fn test_twinkle_config_find() {
    let json = CONFIG_JSON;
    let mut config = TwinkleConfig::from_str(json).unwrap();

    let repo_path = Path::new("/Users/hbons/Projects/twinkle/TwinkleTest");
    let repo = config.find(repo_path).unwrap();

    assert_eq!(repo.path, repo_path);

    let repo_path = Path::new("/Users/hbons/Projects/twinkle/TwinkleTest 2");
    let result = config.find(repo_path);

    assert!(result.is_err());
}


#[test]
fn test_twinkle_config_add() {
    let json = CONFIG_JSON;
    let mut config = TwinkleConfig::from_str(json).unwrap();

    let mut repo = TwinkleRepository::default();
    repo.path = PathBuf::from("/Users/hbons/Projects/twinkle/TwinkleTest 2");
    _ = config.add(&repo).unwrap();

    assert_eq!(config.list().unwrap().len(), 2);

    let result = config.add(&repo);
    assert!(result.is_err());
}


#[test]
fn test_twinkle_config_remove() {
    let json = CONFIG_JSON;
    let mut config = TwinkleConfig::from_str(json).unwrap();

    let path = PathBuf::from("/Users/hbons/Projects/twinkle/TwinkleTest");
    _ = config.remove(&path).unwrap();

    assert_eq!(config.list().unwrap().len(), 0);

    let result = config.remove(&path);
    assert!(result.is_err());
}


#[test]
fn test_twinkle_config_list() {
    let json = CONFIG_JSON;
    let config = TwinkleConfig::from_str(json).unwrap();

    assert_eq!(config.list().unwrap().len(), 1);
}


#[test]
fn test_twinkle_config_set_user() {
    let json = CONFIG_JSON;
    let mut config = TwinkleConfig::from_str(json).unwrap();

    let repo_path = Path::new("/Users/hbons/Projects/twinkle/TwinkleTest");
    _ = config.set_user(repo_path, Some("Hylke Bons"), Some("hello@planetpeanut.studio"));

    let repo = config.find(repo_path).unwrap();
    assert_eq!(repo.user.to_string(), "Hylke Bons <hello@planetpeanut.studio>");

    let result = config.set_user(repo_path, Some("Hylke Bons"), Some(""));
    assert!(result.is_err());
}


#[test]
fn test_twinkle_config_set_interval() {
    let json = CONFIG_JSON;
    let mut config = TwinkleConfig::from_str(json).unwrap();

    let repo_path = Path::new("/Users/hbons/Projects/twinkle/TwinkleTest");
    config.set_interval(repo_path, 42).unwrap();

    let repo = config.find(repo_path).unwrap();
    assert_eq!(repo.polling_interval, Some(42));
}
