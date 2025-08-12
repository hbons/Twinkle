//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::ssh::keygen::{ ssh_keygen, ssh_keygen_fingerprint };
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::keys::host_key::HostKey;


#[test]
fn test_ssh_keygen() {
    let keys_dir = Path::new("./src/tests/.tmp");
    let key_path = keys_dir.join("test.key");

    if keys_dir.exists() {
        fs::remove_dir_all(keys_dir).unwrap();
    }

    let key_pair = ssh_keygen(&key_path, KeyType::ED25519, 256).unwrap();

    assert_eq!(key_pair.key_type, KeyType::ED25519);
    assert!(key_pair.private_key_path.exists());
    assert!(key_pair.private_key.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
    assert!(key_pair.public_key_path.exists());
    assert!(key_pair.public_key.starts_with("ssh-ed25519"));

    _ = fs::remove_dir_all(keys_dir);
}


#[test]
fn test_ssh_keygen_fingerprint() {
    thread::sleep(Duration::from_secs(1));

    let host_key = HostKey {
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl".into(),
        ..Default::default()
    };

    let fingerprint = ssh_keygen_fingerprint(&host_key).unwrap();
    assert!(fingerprint.to_string().starts_with("SHA256:"));
    assert_eq!(fingerprint.to_string().len(), 50);
}
