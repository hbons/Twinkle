//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_sourceforge() -> HostKey {
    // Last updated: June 3, 2026
    // Source:

    HostKey {
        host: "git.code.sf.net".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIGObtXLh/mZom0pXjE5Mu211O+JvtzolqdNKVA+XJ466".into(),
        fingerprint: Some(Fingerprint::SHA256("vDwNztsrZFViJXWpUTSKGo8cF6n79iKAURNiK68n/yE".into())),
    }
}
