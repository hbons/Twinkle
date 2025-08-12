//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fs;
use std::path::Path;

use crate::twinkle::twinkle_util::twinkle_unique_dir;


#[test]
fn test_twinkle_unique_dir() {
    let tmp_dir = Path::new("./src/tests/.tmp");
    fs::create_dir_all(tmp_dir).unwrap();


    let dir = Path::new("./src/tests/.tmp/folder");
    let dir = twinkle_unique_dir(dir);

    assert_eq!(Path::new("./src/tests/.tmp/folder"), dir);


    fs::create_dir_all("./src/tests/.tmp/folder").unwrap();
    let dir = Path::new("./src/tests/.tmp/folder");
    let dir = twinkle_unique_dir(dir);

    assert_eq!(Path::new("./src/tests/.tmp/folder 2"), dir);


    fs::create_dir_all("./src/tests/.tmp/folder 2").unwrap();
    let dir = Path::new("./src/tests/.tmp/folder");
    let dir = twinkle_unique_dir(dir);

    assert_eq!(Path::new("./src/tests/.tmp/folder 3"), dir);
}
