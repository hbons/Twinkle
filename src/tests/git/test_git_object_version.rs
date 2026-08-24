//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::git::objects::version::GitVersion;


#[test]
fn test_git_object_version_new() {
    assert!(GitVersion::new().is_some());
}


#[test]
fn test_git_object_version_from_str() {
    let s = "git version 3.14";
    assert_eq!(s.parse::<GitVersion>().unwrap(), GitVersion::Git3(Some("3.14".into())));

    let s = "git version 2.14";
    assert_eq!(s.parse::<GitVersion>().unwrap(), GitVersion::Git2(Some("2.14".into())));

    let s = "git version 2.39.5 (Apple Git-154)";
    assert_eq!(s.parse::<GitVersion>().unwrap(), GitVersion::Git2(Some("2.39.5 (Apple Git-154)".into())));

    let s = "git 2.14";
    assert!(s.parse::<GitVersion>().is_err());

    let s = "2.14";
    assert!(s.parse::<GitVersion>().is_err());

    let s = " ";
    assert!(s.parse::<GitVersion>().is_err());

    let s = "";
    assert!(s.parse::<GitVersion>().is_err());
}


#[test]
fn test_git_object_version_to_string() {
    assert_eq!(GitVersion::Git2(Some("2.14".into())).to_string(), "git version 2.14");
    assert_eq!(GitVersion::Git3(Some("3.14".into())).to_string(), "git version 3.14");
    assert_eq!(GitVersion::Git2(None).to_string(), "git version 2");
    assert_eq!(GitVersion::Git3(None).to_string(), "git version 3");
}


#[test]
fn test_git_object_version_default() {
    assert_eq!(GitVersion::default(), GitVersion::Git2(None));
}
