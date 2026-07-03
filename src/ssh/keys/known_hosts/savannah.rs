//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_savannah() -> HostKey {
    // Last updated: July 3, 2026
    // Source: https://savannah.gnu.org/maintenance/SshAccess/

    HostKey {
        host: "git.savannah.gnu.org".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIMnMLHxGS/b6Su98mL/J58FkpEJY/X1mONqhPBuFX5sJ".into(),
        fingerprint: Some(Fingerprint::SHA256("o/oI4CKKcWc4cZvDFEdmOXsE3tiPP8bWa04h4bQjtV4".into())),
    }
}
