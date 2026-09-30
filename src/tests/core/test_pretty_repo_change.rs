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
use crate::git::objects::id::GitId;
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
    let id = "3e1e0256967194f0aa3abae31114f56f034965b3".parse::<GitId>().unwrap();
    let is_merge = true;
    let dot = util::cli_dimmed("•");
    let merge = format!(" {dot} {}", util::cli_cyan("merge"));
    let letter = format_file_status(&change.status_x.clone().unwrap());
    let short_id = id.to_short_str();

    let s = format_repo_change(&url, &id, is_merge, &change, "!").unwrap();
    assert_eq!(s, format!("github.com:hbons/Twinkle ! {short_id}{merge} {dot} {letter} `test.txt`"));

    let is_merge = false;
    let s = format_repo_change(&url, &id, is_merge, &change, "!").unwrap();
    assert_eq!(s, format!("github.com:hbons/Twinkle ! {short_id} {dot} {letter} `test.txt`"));

    let change = GitChange::default();
    let r = format_repo_change(&url, &id, is_merge, &change, "!");
    assert!(r.is_none());
}
