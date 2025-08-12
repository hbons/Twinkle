//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;

use crate::git::objects::change::GitChange;
use crate::git::objects::file_status::GitFileStatus;
use crate::twinkle::twinkle_util::twinkle_commit_message;


#[test]
fn test_twinkle_commit_message() {
    let empty_status = vec![];
    assert_eq!(twinkle_commit_message(&empty_status), None);

    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        }
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+ \"test.txt\"".to_string()));
}


#[test]
fn test_twinkle_commit_message_renamed() {
    let empty_status = vec![];
    assert_eq!(twinkle_commit_message(&empty_status), None);

    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Renamed(Path::new("test.txt").into())),
            status_y: None,
            path: "test.txt".into(),
        }
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+1, −1".to_string()));
}


#[test]
fn test_twinkle_commit_message_copied() {
    let empty_status = vec![];
    assert_eq!(twinkle_commit_message(&empty_status), None);

    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Copied(Path::new("test.txt").into())),
            status_y: None,
            path: "test.txt".into(),
        }
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+ \"test.txt\"".to_string()));
}


#[test]
fn test_twinkle_commit_message_multiple() {
    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        },
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        },
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+2".to_string()));
}


#[test]
fn test_twinkle_commit_message_various_2() {
    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        },
        GitChange {
            status_x: Some(GitFileStatus::Deleted),
            status_y: None,
            path: "test.txt".into(),
        },
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+1, −1".to_string()));
}


#[test]
fn test_twinkle_commit_message_various_3() {
    let status = vec![
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        },
        GitChange {
            status_x: Some(GitFileStatus::Modified),
            status_y: None,
            path: "test.txt".into(),
        },
        GitChange {
            status_x: Some(GitFileStatus::Deleted),
            status_y: None,
            path: "test.txt".into(),
        },
    ];

    assert_eq!(twinkle_commit_message(&status), Some("+1, ~1, −1".to_string()));
}
