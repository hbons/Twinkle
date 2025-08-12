//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str::FromStr;
use crate::git::objects::file_status::GitFileStatus;


#[test]
fn test_git_object_file_status_from_str() {
    assert_eq!(GitFileStatus::from_str("A").unwrap(), GitFileStatus::Added);
    assert_eq!(GitFileStatus::from_str("M").unwrap(), GitFileStatus::Modified);
    assert_eq!(GitFileStatus::from_str("D").unwrap(), GitFileStatus::Deleted);
    assert!(matches!(GitFileStatus::from_str("R").unwrap(), GitFileStatus::Renamed(_)));
    assert!(matches!(GitFileStatus::from_str("C").unwrap(), GitFileStatus::Copied(_)));
    assert_eq!(GitFileStatus::from_str("T").unwrap(), GitFileStatus::TypeChanged);
    assert_eq!(GitFileStatus::from_str("U").unwrap(), GitFileStatus::Unmerged);
    assert_eq!(GitFileStatus::from_str("?").unwrap(), GitFileStatus::Untracked);
    assert_eq!(GitFileStatus::from_str("!").unwrap(), GitFileStatus::Ignored);

    let result = GitFileStatus::from_str("asdfgasdfg");
    assert!(result.is_err());
}


#[test]
fn test_git_object_file_status_display() {
    assert_eq!("A", format!("{}", GitFileStatus::Added));
    assert_eq!("M", format!("{}", GitFileStatus::Modified));
    assert_eq!("D", format!("{}", GitFileStatus::Deleted));
    assert_eq!("R", format!("{}", GitFileStatus::Renamed("test.txt".into())));
    assert_eq!("C", format!("{}", GitFileStatus::Copied("test.txt".into())));
    assert_eq!("T", format!("{}", GitFileStatus::TypeChanged));
    assert_eq!("U", format!("{}", GitFileStatus::Unmerged));
    assert_eq!("?", format!("{}", GitFileStatus::Untracked));
    assert_eq!("!", format!("{}", GitFileStatus::Ignored));
}
