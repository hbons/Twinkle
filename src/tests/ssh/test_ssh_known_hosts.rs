//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::ssh::keys::known_hosts::bitbucket::ssh_hostkey_bitbucket;
use crate::ssh::keys::known_hosts::codeberg::ssh_hostkey_codeberg;
use crate::ssh::keys::known_hosts::gitee::ssh_hostkey_gitee;
use crate::ssh::keys::known_hosts::github::ssh_hostkey_github;
use crate::ssh::keys::known_hosts::gitlab::ssh_hostkey_gitlab;
use crate::ssh::keys::known_hosts::gnome::ssh_hostkey_gnome;
use crate::ssh::keys::known_hosts::savannah::ssh_hostkey_savannah;
use crate::ssh::keys::known_hosts::sourcehut::ssh_hostkey_sourcehut;

use crate::ssh::keyscan::ssh_keyscan;
use crate::ssh::keys::key_type::KeyType;


#[test]
fn test_ssh_known_hostkey_bitbucket() {
    let pinned_key = ssh_hostkey_bitbucket();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_codeberg() {
    let pinned_key = ssh_hostkey_codeberg();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_github() {
    let pinned_key = ssh_hostkey_github();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_gitlab() {
    let pinned_key = ssh_hostkey_gitlab();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_gnome() {
    let mut pinned_key = ssh_hostkey_gnome();
    pinned_key.host = "ssh.gitlab.gnome.org".into(); // SSH url differs

    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_sourcehut() {
    let pinned_key = ssh_hostkey_sourcehut();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_savannah() {
    let pinned_key = ssh_hostkey_savannah();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}


#[test]
fn test_ssh_known_hostkey_gitee() {
    let pinned_key = ssh_hostkey_gitee();
    let remote_key = ssh_keyscan(&pinned_key.host, None, KeyType::ED25519).unwrap();

    assert_eq!(pinned_key.fingerprint, remote_key.fingerprint);
    assert_eq!(pinned_key.public_key, remote_key.public_key);
}
