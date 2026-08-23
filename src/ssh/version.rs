//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::process::Command;


pub fn ssh_version() -> Option<String> {
    Command::new("ssh").arg("-V")
        .output()
        .ok()
        .map(|o|
            String::from_utf8_lossy(
                o.stderr.trim_ascii_end()
            ).into()
        )
}
