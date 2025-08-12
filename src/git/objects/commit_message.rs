//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use std::str;


#[derive(Debug, Default)]
pub struct GitCommitMessage {
    pub title: String,
    pub body: Option<String>,
}


impl str::FromStr for GitCommitMessage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start();
        let mut message = GitCommitMessage::default();

        if let Some((title, body)) = s.split_once("\n") {
            message.title = title.trim().to_string();
            message.body = Some(body.trim().to_string());
        } else {
            message.title = s.trim_start().to_string();
        }

        Ok(message)
    }
}


impl fmt::Display for GitCommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)?;

        if let Some(body) = &self.body {
            write!(f, "\n\n{}", body)?;
        }

        Ok(())
    }
}
