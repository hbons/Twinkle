//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::Path;

use crate::core::pretty::format_dir;


#[test]
fn test_twinkle_pretty_dir() {
    env::set_var("HOME", "/home/hbons");

    let path = Path::new("/home/hbons/Projects");
    let dir = format_dir(path);
    assert_eq!(dir, "~/Projects");

    let path = Path::new("/home/hbons/");
    let dir = format_dir(path);
    assert_eq!(dir, "~/");

    let path = Path::new("/home/hbons");
    let dir = format_dir(path);
    assert_eq!(dir, "~");

    let path = Path::new("");
    let dir = format_dir(path);
    assert_eq!(dir, "");
}
