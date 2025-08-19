//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::str::FromStr;
use crate::git::objects::commit_message::GitCommitMessage;


#[test]
fn test_object_commit_message_from_str() {
    let message = GitCommitMessage::from_str("Initial commit\n\nThis is the body\nAnother line").unwrap();

    assert_eq!(message.title, "Initial commit");
    assert_eq!(message.body, Some("This is the body\nAnother line".to_string()));


    let message = GitCommitMessage::from_str("Initial commit").unwrap();

    assert_eq!(message.title, "Initial commit");
    assert_eq!(message.body, None);
}


#[test]
fn test_object_commit_message_to_string() {
    let message = GitCommitMessage {
        title: "Initial commit".to_string(),
        body: Some("This is the body".to_string()),
    };

    assert_eq!(message.to_string(), "Initial commit\n\nThis is the body");


    let message = GitCommitMessage {
        title: "Initial commit".to_string(),
        body: None,
    };

    assert_eq!(message.to_string(), "Initial commit");
}
