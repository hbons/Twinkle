//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str::FromStr;
use crate::git::objects::merge_status::GitMergeStatus;


#[test]
fn test_git_object_merge_status_from_str() {
    assert_eq!(GitMergeStatus::from_str("DD").unwrap(), GitMergeStatus::DD);
    assert_eq!(GitMergeStatus::from_str("AU").unwrap(), GitMergeStatus::AU);
    assert_eq!(GitMergeStatus::from_str("UD").unwrap(), GitMergeStatus::UD);
    assert_eq!(GitMergeStatus::from_str("UA").unwrap(), GitMergeStatus::UA);
    assert_eq!(GitMergeStatus::from_str("DU").unwrap(), GitMergeStatus::DU);
    assert_eq!(GitMergeStatus::from_str("AA").unwrap(), GitMergeStatus::AA);
    assert_eq!(GitMergeStatus::from_str("UU").unwrap(), GitMergeStatus::UU);
    assert_eq!(GitMergeStatus::from_str("??").unwrap(), GitMergeStatus::QQ);
    assert_eq!(GitMergeStatus::from_str("!!").unwrap(), GitMergeStatus::XX);

    let result = GitMergeStatus::from_str("asdfgasdfg");
    assert!(result.is_err());

    let result = GitMergeStatus::from_str("");
    assert!(result.is_err());

    let result = GitMergeStatus::from_str("M ");
    assert!(result.is_err());
}
