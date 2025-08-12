//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_bitbucket() -> HostKey {
    // Last updated: March 9, 2025
    // Source: https://support.atlassian.com/bitbucket-cloud/docs/configure-ssh-and-two-step-verification

    HostKey {
        host: "bitbucket.org".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIIazEu89wgQZ4bqs3d63QSMzYVa0MuJ2e2gKTKqu+UUO".into(),
        fingerprint: Some(Fingerprint::SHA256("ybgmFkzwOSotHTHLJgHO0QN8L0xErw6vd0VhFA9m3SM".into())),
    }
}
