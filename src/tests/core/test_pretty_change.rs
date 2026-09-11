//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::PathBuf;

use crate::cli::util::cli_dimmed;
use crate::git::objects::change::GitChange;
use crate::git::objects::status::GitFileStatus;
use crate::core::pretty::format_change;


#[test]
fn test_pretty_change() {
    let change = GitChange {
        status_x: Some(GitFileStatus::Added),
        ..Default::default()
    };

    assert_eq!(format_change(&change), format!("`{}`", change.path.display()));


    let change = GitChange {
        status_x: Some(GitFileStatus::Copied(Some(PathBuf::from("README.md")))),
        status_y: None,
        path: PathBuf::from("README2.md"),
    };

    let arrow = cli_dimmed("→");
    assert_eq!(format_change(&change), format!("`README.md` {arrow} `README2.md`"));


    let change = GitChange {
        status_x: Some(GitFileStatus::Renamed(Some(PathBuf::from("README.md")))),
        status_y: None,
        path: PathBuf::from("README2.md"),
    };

    let arrow = cli_dimmed("→");
    assert_eq!(format_change(&change), format!("`README.md` {arrow} `README2.md`"));


    let change = GitChange {
        status_x: Some(GitFileStatus::Copied(None)),
        status_y: None,
        path: PathBuf::from("README2.md"),
    };

    assert_eq!(format_change(&change), format!("`README2.md`"));


    let change = GitChange {
        status_x: Some(GitFileStatus::Renamed(None)),
        status_y: None,
        path: PathBuf::from("README2.md"),
    };

    assert_eq!(format_change(&change), format!("`README2.md`"));
}
