//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::twinkle_pretty::twinkle_pretty_bool;


#[test]
fn test_twinkle_pretty_bool() {
    let pretty_value = twinkle_pretty_bool(true);
    assert_eq!(pretty_value, "Yes".to_string());

    let pretty_value = twinkle_pretty_bool(false);
    assert_eq!(pretty_value, "No".to_string());
}
