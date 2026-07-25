//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::current_dir;
use std::error::Error;


use crate::app::App;

use crate::ssh::objects::url::SshUrl;
use crate::twinkle::twinkle_init::twinkle_init;



impl App {
    pub fn cli_command_init(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>>{
        self.cli_require_args(2, args).map_err(|_| {
            Self::cli_command_init_usage();
            "Missing <user@host:path>"
        })?;

        let ssh_url = args.get(2).ok_or("Missing <user@host:path>")?;
        let ssh_url = ssh_url.parse::<SshUrl>().map_err(|_| {
            Self::cli_command_init_usage();
            "Not a valid <user@host:path>"
        })?;

        let path = current_dir()?;
        twinkle_init(&path, &ssh_url, None)?;

        Ok(())
    }

    fn cli_command_init_usage() {
        println!("Usage: twinkle init <user@host:path> [path]");
        println!("               init <ssh://user@host[:port]/path> [path]");
        println!();
    }
}
