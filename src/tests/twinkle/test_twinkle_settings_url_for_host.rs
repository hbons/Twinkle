//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::twinkle_util::twinkle_settings_url_for;


#[test]
fn test_settings_url_for() {
    let url = twinkle_settings_url_for("github.com".to_string());
    assert!(url.is_some());

    let url = twinkle_settings_url_for("asdfgasdfg".to_string());
    assert!(url.is_none());
}
