//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use crate::core::defaults::hosts::host_ssh_settings_url;


#[test]
fn test_settings_url_for() {
    let url = host_ssh_settings_url("github.com".to_string());
    assert!(url.is_some());

    let url = host_ssh_settings_url("asdfgasdfg".to_string());
    assert!(url.is_none());
}
