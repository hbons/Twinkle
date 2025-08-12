//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_gitlab() -> HostKey {
    // Last updated: March 6, 2025
    // Source: https://docs.gitlab.com/17.5/user/gitlab_com/#ssh-host-keys-fingerprints

    HostKey {
        host: "gitlab.com".into(),
        is_trusted: true,
        key_type: KeyType::ED25519,
        public_key: "AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf".into(),
        fingerprint: Some(Fingerprint::SHA256("eUXGGm1YGsMAS7vkcx6JOJdOGHPem5gQp4taiCfCLB8".into())),
    }
}
