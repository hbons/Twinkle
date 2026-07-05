//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_kde() -> HostKey {
    // Last updated: July 3, 2026
    // Source: https://community.kde.org/Infrastructure/Git

    HostKey {
        host: "invent.kde.org".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIOMtd90DMLrtdCiapQK43JjwKk+U8egSXQU15fOJba1n".into(),
        fingerprint: Some(Fingerprint::SHA256("zHdK2R/S6s5Oj71N0s8LHWCXXsUt+DCztd+GjzW9KlU".into())),
    }
}
