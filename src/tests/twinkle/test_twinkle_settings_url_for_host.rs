//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::defaults::hosts::twinkle_host_ssh_settings_url;


#[test]
fn test_settings_url_for() {
    let url = twinkle_host_ssh_settings_url("github.com".to_string());
    assert!(url.is_some());

    let url = twinkle_host_ssh_settings_url("asdfgasdfg".to_string());
    assert!(url.is_none());
}
