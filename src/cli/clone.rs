//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::app::App;
use crate::log;
use crate::ssh::objects::url::SshUrl;
use crate::twinkle::twinkle_clone::{ twinkle_clone_complete, twinkle_clone_start };


impl App {
    fn cli_command_clone_usage() {
        println!("Usage: twinkle clone <user@host:path> [path]");
        println!("               clone <ssh://user@host[:port]/path> [path]");
        println!();
    }


    pub fn cli_command_clone(
        &mut self,
        args: &Vec<String>
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(2, args).map_err(|_| {
            Self::cli_command_clone_usage();
            "Missing <user@host:path>"
        })?;

        let ssh_url = args.get(2)
            .ok_or("Missing <user@host:path>")?;

        let ssh_url = ssh_url.parse::<SshUrl>().map_err(|_| {
            Self::cli_command_clone_usage();
            "Not a valid <user@host:path>"
        })?;

        let path = match args.get(3) {
            Some(s) => PathBuf::from(s),
            None => current_dir()?,
        };

        let path = fs::canonicalize(path).map_err(|_| {
            Self::cli_command_clone_usage();
            "Not a valid <path>"
        })?;

        let mut repo = twinkle_clone_start(&ssh_url, None, &path)?;
        twinkle_clone_complete(&mut repo, None)?;

        if repo.git.lfs_version().is_none() {
            log::warning("git-lfs command not found");
        }

        Ok(())

        // loop {
        //     match twinkle_clone_prepare(&ssh_url, &self.app_keys_dir) {
        //         Err(e) => match e.downcast_ref::<TwinkleCloneError>() {
        //             Some(TwinkleCloneError::NeedsNetwork) => {
        //                 println!("Could not connect to {}", ssh_url.host);
        //             },
        //             Some(TwinkleCloneError::NeedsTrust(host_key)) => {
        //                 let fingerprint = host_key.fingerprint.clone().unwrap_or_else(|| {
        //                     log::error_and_exit("No host key fingerprint");
        //                 });

        //                 println!("Connected to {} {} {} ",
        //                     host_key.host, cli_dimmed("–"), cli_dimmed(&fingerprint.to_string()));

        //                 io::stdout().flush()?;
        //                 twinkle_hostkey_trust(host_key, &self.app_keys_dir)?;
        //             },
        //             Some(TwinkleCloneError::NeedsAuth(host_key, key_pair)) => {
        //                 let url = twinkle_settings_url_for(host_key.host.clone());
        //                 let url = url.unwrap_or(&host_key.host);
        //                 let settings_url = cli_link(url, None);

        //                 println!();
        //                 println!("First, add this SSH key to {settings_url}:");
        //                 println!("{}", cli_bold(&key_pair.public_key));
        //                 println!();
        //                 print!("Then, press {enter} to clone {url}… ",
        //                     enter=cli_bold("[Enter]"),
        //                     url=cli_bold(&ssh_url.original));

        //                 io::stdout().flush()?;
        //                 let mut input = String::new();
        //                 io::stdin().read_line(&mut input)?;
        //             },
        //             None => return Err(e),
        //         },
        //         Ok(key_pair) => {
        //             let mut repo = twinkle_clone_start(&ssh_url, &key_pair, &path)?;
        //             twinkle_clone_complete(&mut repo, &key_pair)?;
        //             self.config.add(&repo)?;

        //             return Ok(());
        //         }
        //     };

        //     thread::sleep(Duration::from_millis(500));
        // }
    }
}
