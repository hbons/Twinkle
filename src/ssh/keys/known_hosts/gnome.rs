//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_gnome() -> HostKey {
    // Last updated: March 10, 2025
    // Source: https://gitlab.gnome.org/help/instance_configuration
    // Note: Host keys are at ssh.gitlab.gnome.org

    HostKey {
        host: "gitlab.gnome.org".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIHG6b3deoYMPwKEu9Sj+y6MBHYYUKQiAnta/go3aNv7R".into(),
        fingerprint: Some(Fingerprint::SHA256("Y9G4dWiIfi53LR3InJWsIbv4lekUHonq/HrqTTm/rcw".into())),
    }
}
