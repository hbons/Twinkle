//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::process::Command;


pub fn ssh_version() -> String {
    match Command::new("ssh").arg("-V").output() {
        Ok(output) => String::from_utf8_lossy(&output.stderr).trim().to_string(),
        Err(_)     => "\x1b[33mOpenSSH not found\x1b[0m".to_string(),
    }
}
