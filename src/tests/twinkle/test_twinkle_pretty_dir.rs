//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::Path;

use crate::twinkle::twinkle_pretty::twinkle_pretty_dir;


#[test]
fn test_twinkle_pretty_dir() {
    env::set_var("HOME", "/home/hbons");

    let path = Path::new("/home/hbons/Projects");
    let dir = twinkle_pretty_dir(path);
    assert_eq!(dir, "~/Projects");

    let path = Path::new("/home/hbons/");
    let dir = twinkle_pretty_dir(path);
    assert_eq!(dir, "~/");

    let path = Path::new("/home/hbons");
    let dir = twinkle_pretty_dir(path);
    assert_eq!(dir, "~");

    let path = Path::new("");
    let dir = twinkle_pretty_dir(path);
    assert_eq!(dir, "");
}
