//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_codeberg() -> HostKey {
    // Last updated: March 6, 2025
    // Source: https://codeberg.org/Codeberg/org/src/branch/main/Imprint.md#user-content-ssh-fingerprints
    //         https://docs.codeberg.org/security/ssh-fingerprint/

    HostKey {
        host: "codeberg.org".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIIVIC02vnjFyL+I4RHfvIGNtOgJMe769VTF1VR4EB3ZB".into(),
        fingerprint: Some(Fingerprint::SHA256("mIlxA9k46MmM6qdJOdMnAQpzGxF4WIVVL+fj+wZbw0g".into())),
    }
}
