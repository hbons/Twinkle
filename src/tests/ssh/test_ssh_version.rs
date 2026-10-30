//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use crate::ssh::util::version;


#[test]
fn test_ssh_version() {
    let ssh_version = version().unwrap();
    assert!(ssh_version.starts_with("OpenSSH"));
}
