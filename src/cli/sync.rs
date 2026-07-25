//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
// use std::io::{ self, Write };
use std::path::{ Path,};
use std::time::Duration;
// use std::thread;
// use std::time::Duration;

use crate::app::App;
use crate::log;


use crate::twinkle::objects::repository::TwinkleRepository;
use crate::twinkle::twinkle_sync::twinkle_sync;
// use crate::twinkle::twinkle_clone::twinkle_clone_prepare;
// use crate::twinkle::twinkle_clone::twinkle_clone_start;
// use crate::twinkle::twinkle_clone::twinkle_clone_complete;
// use crate::twinkle::twinkle_keys::twinkle_hostkey_trust;
use crate::twinkle::twinkle_pretty::{ twinkle_pretty_dir };
// use crate::twinkle::twinkle_util::twinkle_settings_url_for;


use super::util::*;


impl App {

    pub fn cli_command_sync(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        let interval = args.get(3)
            .and_then(|s| s.strip_prefix("--interval="))
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        let mut repo = TwinkleRepository::new(&path);

        if !repo.enabled() {
            return Err("Repository is disabled".into());
        }

        // TODO: Stop if no user set or let git commit fail?

        let dir = twinkle_pretty_dir(&repo.path);
        let remote_url = repo.remote_url().ok_or("Missing remote_url")?;
        let remote = cli_dimmed(&format!("– {}…\n", remote_url.original));
        let once = false;

        log::log(&format!("Syncing {} {}", cli_bold(&dir), remote));
        twinkle_sync(&mut repo, interval, once)
    }
}
