//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::ssh::keys::key_type::KeyType;


#[test]
fn test_ssh_keytype_fmt() {
    assert_eq!(KeyType::ECDSA.to_string(), "ecdsa-sha2-nistp256");
    assert_eq!(KeyType::ED25519.to_string(), "ed25519");
    assert_eq!(KeyType::RSA.to_string(), "rsa");
}
