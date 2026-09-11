//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::cli::util;
use crate::core::pretty::format_file_status;
use crate::git::objects::file_status::GitFileStatus;


#[test]
fn test_pretty_file_status() {
    let s = GitFileStatus::Added;
    assert_eq!(format_file_status(&s), util::cli_green("A"));

    let s = GitFileStatus::Copied(None);
    assert_eq!(format_file_status(&s), util::cli_green("C"));

    let s = GitFileStatus::Modified;
    assert_eq!(format_file_status(&s), util::cli_yellow("M"));

    let s = GitFileStatus::Renamed(None);
    assert_eq!(format_file_status(&s), util::cli_yellow("R"));

    let s = GitFileStatus::TypeChanged;
    assert_eq!(format_file_status(&s), util::cli_yellow("T"));

    let s = GitFileStatus::Unmerged;
    assert_eq!(format_file_status(&s), util::cli_red("U"));

    let s = GitFileStatus::Deleted;
    assert_eq!(format_file_status(&s), util::cli_red("D"));

    let s = GitFileStatus::Ignored;
    assert_eq!(format_file_status(&s), "!");

    let s = GitFileStatus::Untracked;
    assert_eq!(format_file_status(&s), "?");
}
