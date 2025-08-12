//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::process::Command;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-version

    pub fn version(&self) -> String {
        match Command::new("git").arg("--version").output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().into(),
            Err(_) => "\x1b[33mGit not found\x1b[0m".into(),
        }
    }
}
