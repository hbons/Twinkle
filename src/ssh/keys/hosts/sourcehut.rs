//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_sourcehut() -> HostKey {
    // Last updated: March 26, 2025
    // Source: https://man.sr.ht/git.sr.ht/#ssh-host-keys

    HostKey {
        host: "git.sr.ht".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIMZvRd4EtM7R+IHVMWmDkVU3VLQTSwQDSAvW0t2Tkj60".into(),
        fingerprint: Some(Fingerprint::SHA256("WXXNZu0YyoE3KBl5qh4GsnF1vR0NeEPYJAiPME+P09g".into())),
    }
}
