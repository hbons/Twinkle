//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;
use std::str::FromStr;

use crate::twinkle::twinkle_resolve::twinkle_resolve_path_names;
use crate::git::objects::user::GitUser;


#[test]
fn test_twinkle_resolve_paths() {
    let user       = GitUser::from_str("Hylke Bons <hi@planetpeanut.studio>").unwrap();
    let user_same  = GitUser::from_str("Hylke Bons <hi@planetpeanut.studio>").unwrap();
    let user_other = GitUser::from_str("Hylke <hi@planetpeanut.studio>").unwrap();

    let path1 = Path::new("file");
    let path2 = Path::new("file.txt");
    let path3 = Path::new("/path/to/file");
    let path4 = Path::new("/path/to/file.txt");


    // Same users
    let (path_a, path_b) = twinkle_resolve_path_names(path1, &user, &user_same).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "file (A)");
    assert_eq!(path_b.to_str().unwrap(), "file (B)");

    let (path_a, path_b) = twinkle_resolve_path_names(path2, &user, &user_same).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "file (A).txt");
    assert_eq!(path_b.to_str().unwrap(), "file (B).txt");

    let (path_a, path_b) = twinkle_resolve_path_names(path3, &user, &user_same).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "/path/to/file (A)");
    assert_eq!(path_b.to_str().unwrap(), "/path/to/file (B)");

    let (path_a, path_b) = twinkle_resolve_path_names(path4, &user, &user_same).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "/path/to/file (A).txt");
    assert_eq!(path_b.to_str().unwrap(), "/path/to/file (B).txt");


    // Different users
    let (path_a, path_b) = twinkle_resolve_path_names(path1, &user, &user_other).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "file (Hylke Bons)");
    assert_eq!(path_b.to_str().unwrap(), "file (Hylke)");

    let (path_a, path_b) = twinkle_resolve_path_names(path2, &user, &user_other).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "file (Hylke Bons).txt");
    assert_eq!(path_b.to_str().unwrap(), "file (Hylke).txt");

    let (path_a, path_b) = twinkle_resolve_path_names(path3, &user, &user_other).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "/path/to/file (Hylke Bons)");
    assert_eq!(path_b.to_str().unwrap(), "/path/to/file (Hylke)");

    let (path_a, path_b) = twinkle_resolve_path_names(path4, &user, &user_other).unwrap();
    assert_eq!(path_a.to_str().unwrap(), "/path/to/file (Hylke Bons).txt");
    assert_eq!(path_b.to_str().unwrap(), "/path/to/file (Hylke).txt");
}
