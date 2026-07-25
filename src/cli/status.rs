//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;
// use std::thread;
// use std::time::Duration;

use crate::app::App;
// use crate::twinkle::twinkle_clone::twinkle_clone_prepare;
// use crate::twinkle::twinkle_clone::twinkle_clone_start;
// use crate::twinkle::twinkle_clone::twinkle_clone_complete;
// use crate::twinkle::twinkle_keys::twinkle_hostkey_trust;
use crate::twinkle::twinkle_pretty::{  twinkle_pretty_bool, twinkle_pretty_datetime, twinkle_pretty_dir };
// use crate::twinkle::twinkle_util::twinkle_settings_url_for;


use super::util::*;

use crate::twinkle::objects::repository::TwinkleRepository;

impl App {
    pub fn cli_command_status(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        dbg!(&path);

        let repo = TwinkleRepository::new(&path);
        let path = twinkle_pretty_dir(&path);

        println!();
        println!("       {} {}", cli_dimmed("Path:"), cli_bold(&path));
        println!("     {} {}", cli_dimmed("Remote:"), repo.remote_url().map(|u| u.to_string()).unwrap_or_else(|| "–".to_string()));
        println!("     {} {}", cli_dimmed("Branch:"), repo.branch().unwrap_or("err".to_string()));
        println!();
        println!("    {} {}", cli_dimmed("Enabled:"), repo.enabled());
        println!("         {} {}", cli_dimmed("ID:"), repo.id().unwrap_or("–".into()));
        println!("        {} {}", cli_dimmed("LFS:"), twinkle_pretty_bool(repo.lfs_enabled()));
        // println!("       {} {}", cli_dimmed("User:"), repo.user().unwrap_or("default"));
        println!();
        println!(" {} {}", cli_dimmed("Last check:"), twinkle_pretty_datetime(repo.last_checked().unwrap_or(0)));
        println!("  {} {}", cli_dimmed("Last sync:"), twinkle_pretty_datetime(repo.last_synced().unwrap_or(0)));
        println!("   {} {}s", cli_dimmed("Interval:"), repo.polling_interval().as_secs());
        println!();

        Ok(())
    }
}
