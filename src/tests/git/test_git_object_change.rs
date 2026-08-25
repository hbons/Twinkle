//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::ffi::OsStr;
use std::path::Path;

use crate::git::objects::change::GitChange;
use crate::git::objects::status::GitFileStatus;
use crate::git::objects::status::GitMergeStatus;


#[test]
fn test_object_change_from_status_line() {
    let input1 = OsStr::new("A  src/git.rs");
    let input2 = OsStr::new("AM src/git.rs");
    let input3 = OsStr::new("RM src/git 2.rs\0src/git.rs");
    let input4 = OsStr::new("RM src/git 2.rs\0src/git.rs");
    let input5 = OsStr::new(" D src/git.rs");

    let change = GitChange::from_status_line(&input1).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.status_y, None);
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_status_line(input2).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_status_line(input3).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(Some(orig_path))));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_status_line(input4).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(Some(orig_path))));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_status_line(input5).unwrap();
    assert_eq!(change.status_x, None);
    assert_eq!(change.status_y, Some(GitFileStatus::Deleted));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());


    // Errors
    let input1 = OsStr::new("");
    let input2 = OsStr::new("asdfgasdfgasdfg");
    let input3 = OsStr::new("AAAsrc/git.rs");
    let input4 = OsStr::new("A src/git.rs");

    assert!(GitChange::from_status_line(input1).is_err());
    assert!(GitChange::from_status_line(input2).is_err());
    assert!(GitChange::from_status_line(input3).is_err());
    assert!(GitChange::from_status_line(input4).is_err());
}


#[test]
fn test_object_change_from_log_line() {
    let input1 = OsStr::new("A\0src/git.rs");
    let input2 = OsStr::new("M\0src/git.rs");
    let input3 = OsStr::new("D\0src/git.rs");
    let input4 = OsStr::new("R100\0src/git.rs\0src/git 2.rs");
    let input5 = OsStr::new("C100\0src/git.rs\0src/git 2.rs");

    let change = GitChange::from_log_line(input1).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_log_line(input2).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_log_line(input3).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Deleted));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_log_line(input4).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(Some(orig_path))));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_log_line(input5).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Copied(Some(orig_path))));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());


    // Errors
    let input1 = OsStr::new("");
    let input2 = OsStr::new("asdfgasdfgasdfg");
    let input3 = OsStr::new("\tasdfgasdfgasdfg");
    let input4 = OsStr::new("asdfgasdfgasdfg\t");

    assert!(GitChange::from_log_line(input1).is_err());
    assert!(GitChange::from_log_line(input2).is_err());
    assert!(GitChange::from_log_line(input3).is_err());
    assert!(GitChange::from_log_line(input4).is_err());
}


#[test]
fn test_git_object_change_as_merge_status() {
    assert!(GitChange::default().as_merge_status().is_none());

    let mut change = GitChange::default();
    change.status_x = Some(GitFileStatus::Added);
    change.status_y = Some(GitFileStatus::Added);
    assert_eq!(change.as_merge_status().unwrap(), GitMergeStatus::AA);

    let mut change = GitChange::default();
    change.status_x = Some(GitFileStatus::Added);
    change.status_y = None;
    assert!(change.as_merge_status().is_none());

    let mut change = GitChange::default();
    change.status_x = None;
    change.status_y = Some(GitFileStatus::Added);
    assert!(change.as_merge_status().is_none());
}
