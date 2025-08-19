//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str::FromStr;
use std::path::Path;

use crate::git::objects::change::GitChange;
use crate::git::objects::file_status::GitFileStatus;
use crate::git::objects::merge_status::GitMergeStatus;


#[test]
fn test_object_change_from_str() {
    let input1 = "R  src/git.rs -> src/git2.rs";
    let input2 = "R100\tsrc/git.rs\tsrc/git2.rs";
    let change1 = GitChange::from_str(input1).unwrap();
    let change2 = GitChange::from_str(input2).unwrap();

    assert_eq!(change1, change2);
}


#[test]
fn test_object_change_from_status_line() {
    let input1 = "A  src/git.rs";
    let input2 = "AM \"src/git.rs\"";
    let input3 = "RM src/git.rs -> src/git 2.rs";
    let input4 = "RM src/git.rs -> \"src/git 2.rs\"";
    let input5 = " D src/git.rs";

    let change = GitChange::from_str(input1).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.status_y, None);
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_str(input2).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_str(input3).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(orig_path)));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_str(input4).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(orig_path)));
    assert_eq!(change.status_y, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_str(input5).unwrap();
    assert_eq!(change.status_x, None);
    assert_eq!(change.status_y, Some(GitFileStatus::Deleted));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());


    // Errors
    let input1 = "";
    let input2 = "asdfgasdfgasdfg";
    let input3 = "AAAsrc/git.rs";
    let input4 = "A src/git.rs";

    assert!(GitChange::from_str(input1).is_err());
    assert!(GitChange::from_str(input2).is_err());
    assert!(GitChange::from_str(input3).is_err());
    assert!(GitChange::from_str(input4).is_err());
}


#[test]
fn test_object_change_from_log_line() {
    let input1 = "A\tsrc/git.rs";
    let input2 = "M\tsrc/git.rs";
    let input3 = "D\tsrc/git.rs";
    let input4 = "R100\tsrc/git.rs\tsrc/git 2.rs";
    let input5 = "C\tsrc/git.rs\tsrc/git 2.rs";

    let change = GitChange::from_str(input1).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_str(input2).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_str(input3).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Deleted));
    assert_eq!(change.path, Path::new("src/git.rs").to_path_buf());

    let change = GitChange::from_str(input4).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(orig_path)));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());

    let change = GitChange::from_str(input5).unwrap();
    let orig_path = Path::new("src/git.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Copied(orig_path)));
    assert_eq!(change.path, Path::new("src/git 2.rs").to_path_buf());


    // Quotes and spaces
    let input1 = "A\t\"src/git stuff.rs\"";
    let input2 = "M\t\"src/git stuff.rs\"";
    let input3 = "D\t\"src/git stuff.rs\"";
    let input4 = "R100\t\"src/git stuff.rs\"\t\"src/git stuff 2.rs\"";
    let input5 = "C\t\"src/git stuff.rs\"\t\"src/git stuff 2.rs\"";

    let change = GitChange::from_str(input1).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Added));
    assert_eq!(change.path, Path::new("src/git stuff.rs").to_path_buf());

    let change = GitChange::from_str(input2).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Modified));
    assert_eq!(change.path, Path::new("src/git stuff.rs").to_path_buf());

    let change = GitChange::from_str(input3).unwrap();
    assert_eq!(change.status_x, Some(GitFileStatus::Deleted));
    assert_eq!(change.path, Path::new("src/git stuff.rs").to_path_buf());

    let change = GitChange::from_str(input4).unwrap();
    let orig_path = Path::new("src/git stuff.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Renamed(orig_path)));
    assert_eq!(change.path, Path::new("src/git stuff 2.rs").to_path_buf());

    let change = GitChange::from_str(input5).unwrap();
    let orig_path = Path::new("src/git stuff.rs").to_path_buf();
    assert_eq!(change.status_x, Some(GitFileStatus::Copied(orig_path)));
    assert_eq!(change.path, Path::new("src/git stuff 2.rs").to_path_buf());


    // Errors
    let input1 = "";
    let input2 = "asdfgasdfgasdfg";
    let input3 = "\tasdfgasdfgasdfg";
    let input4 = "asdfgasdfgasdfg\t";

    assert!(GitChange::from_str(input1).is_err());
    assert!(GitChange::from_str(input2).is_err());
    assert!(GitChange::from_str(input3).is_err());
    assert!(GitChange::from_str(input4).is_err());
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
