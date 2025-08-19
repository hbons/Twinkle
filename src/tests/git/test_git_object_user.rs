//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str::FromStr;
use crate::git::objects::user::GitUser;


#[test]
fn test_git_object_user_from_str() {
    let user = GitUser::from_str("Hylke Bons <hello@planetpeanut.studio>").unwrap();
    assert_eq!(user.name(), "Hylke Bons");
    assert_eq!(user.email(), "hello@planetpeanut.studio");

    let user = GitUser::from_str("Hylke Bons <@>").unwrap();
    assert_eq!(user.name(), "Hylke Bons");
    assert_eq!(user.email(), "@");

    let result = GitUser::from_str("<hello@planetpeanut.studio>");
    assert!(result.is_err());

    let result = GitUser::from_str("Hylke Bons");
    assert!(result.is_err());

    let result = GitUser::from_str("Hylke Bons <hello@planetpeanut.studio");
    assert!(result.is_err());
}


#[test]
fn test_git_object_user_to_string() {
    let user = GitUser::from_str("Hylke Bons <hello@planetpeanut.studio>").unwrap();
    assert_eq!(user.to_string(), "Hylke Bons <hello@planetpeanut.studio>");
}


#[test]
fn test_git_object_user_default() {
    let user = GitUser::default();
    assert_eq!(user.to_string(), "Unknown <git@localhost>");
}
