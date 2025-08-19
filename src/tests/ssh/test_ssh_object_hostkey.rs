//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fs;
use std::path::Path;

use crate::ssh::keys::host_key::HostKey;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::objects::url::SshUrl;


#[test]
fn test_ssh_hostkey_for_host() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let host_key = HostKey::for_host(&url, KeyType::ED25519).unwrap();

    assert_eq!(host_key.key_type, KeyType::ED25519);
}


#[test]
fn test_ssh_hostkey_to_string() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let mut host_key = HostKey::for_host(&url, KeyType::ED25519).unwrap();

    assert!(host_key.to_string().starts_with("github.com ssh-ed25519 "));
    host_key.key_type = KeyType::ECDSA;
    assert!(host_key.to_string().starts_with("github.com ecdsa-sha2-nistp256 "));
}


#[test]
fn test_ssh_hostkey_to_file_name() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let host_key = HostKey::for_host(&url, KeyType::ED25519).unwrap();

    assert_eq!(host_key.to_file_name(), Path::new("github.com.ed25519.key.host"));
}


#[test]
fn test_ssh_hostkey_from_file() {
    let url = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();
    let host_key = HostKey::for_host(&url, KeyType::ED25519).unwrap();

    let key_path = Path::new("./src/tests/.tmp/github.com.ed25519.key.host");
    let _ = fs::create_dir_all(key_path.parent().unwrap());
    _ = fs::write(key_path, host_key.to_string());

    let host_key = HostKey::from_file(key_path).unwrap();

    assert_eq!(host_key.host, "github.com");
    assert_eq!(host_key.key_type, KeyType::ED25519);
    assert_eq!(host_key.public_key, "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl");
}
