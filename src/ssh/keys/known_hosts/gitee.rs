//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_gitee() -> HostKey {
    // Last updated: July 3, 2026
    // Source: https://help.gitee.com/account/gitees-ssh-key-fingerprints

    HostKey {
        host: "gitee.com".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIEKxHSJ7084RmkJ4YdEi5tngynE8aZe2uEoVVsB/OvYN".into(),
        fingerprint: Some(Fingerprint::SHA256("+ULzij2u99B9eWYFTw1Q4ErYG/aepHLbu96PAUCoV88".into())),
    }
}
