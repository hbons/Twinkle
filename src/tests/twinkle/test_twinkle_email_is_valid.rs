//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::twinkle_util::twinkle_email_is_valid;


#[test]
fn test_twinkle_pretty_dir() {
    let valid = twinkle_email_is_valid("hi@planetpeanut.studio");
    assert!(valid);

    let valid = twinkle_email_is_valid("@");
    assert!(valid);

    let valid = twinkle_email_is_valid("");
    assert!(!valid);

    let valid = twinkle_email_is_valid("asdfgasdfgasdfg");
    assert!(!valid);
}
