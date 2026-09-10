//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::cli::util;
use crate::core::pretty::{
    format_file_status,
    format_repo_change,
};

use crate::git::objects::change::GitChange;
use crate::git::objects::status::GitFileStatus;
use crate::ssh::objects::url::SshUrl;


#[test]
fn test_pretty_repo_change() {
    let change =
        GitChange {
            status_x: Some(GitFileStatus::Added),
            status_y: None,
            path: "test.txt".into(),
        };

    let url = "ssh://git@github.com:22/hbons/Twinkle".parse::<SshUrl>().unwrap();
    let branch = "main".to_string();
    let hash = "97b9e";
    let dot = util::cli_dimmed("•");
    let letter = format_file_status(&change.status_x.clone().unwrap());

    let s = format_repo_change(&url, &branch, hash, &change, "!").unwrap();

    assert_eq!(s, format!("github.com:hbons/Twinkle ! {hash} {dot} {letter} `test.txt`"));


    let change = GitChange::default();
    let r = format_repo_change(&url, &branch, hash, &change, "!");

    assert!(r.is_none());
}
