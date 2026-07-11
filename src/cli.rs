//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env::current_dir;
use std::error::Error;
use std::fs;
// use std::io::{ self, Write };
use std::path::{ Path, PathBuf };
use std::time::Duration;
// use std::thread;
// use std::time::Duration;

use crate::app::{ App, app_deps, app_version };
use crate::git::objects::environment::GitEnvironment;
use crate::log;

use crate::ssh::objects::url::SshUrl;

use crate::twinkle::objects::repository::TwinkleRepository;
use crate::twinkle::twinkle_clone::{ /* TwinkleCloneError, */ twinkle_clone_complete, twinkle_clone_start};
use crate::twinkle::twinkle_init::twinkle_init;
use crate::twinkle::twinkle_sync::twinkle_sync;
// use crate::twinkle::twinkle_clone::twinkle_clone_prepare;
// use crate::twinkle::twinkle_clone::twinkle_clone_start;
// use crate::twinkle::twinkle_clone::twinkle_clone_complete;
// use crate::twinkle::twinkle_keys::twinkle_hostkey_trust;
use crate::twinkle::twinkle_pretty::{  twinkle_pretty_bool, twinkle_pretty_datetime, twinkle_pretty_dir };
// use crate::twinkle::twinkle_util::twinkle_settings_url_for;



impl App {
    pub fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let command = args.get(1).ok_or("Missing <command>")?;

        match command.as_str() {
            "clone"     => self.cli_command_clone(args)?,
            "init"      => self.cli_command_init(args)?,
            "sync"      => self.cli_command_sync(args)?,
            "status"    => self.cli_command_status(args)?, // Not displayed
            "--help"    => self.cli_option_help(),
            "--version" => println!("{}", app_version()),
            "--deps"    => println!("{}", app_deps()),
            "--env"     => println!("{:#?}", self),
            _ => {
                self.cli_option_help();
                return Err("Unknown command".into());
            }
        }

        Ok(())
    }


    pub fn cli_option_help(&self) {
        println!("Usage: twinkle <command> [args…]");
        println!();
        println!("Commands:");
        println!("    clone <user@host:path> [path]");
        println!("    init  <user@host:path> [path]");
        println!("    sync  [path] [--interval=60]");
        println!();
        println!("Support:");
        println!("    {}",
            cli_link(
                "https://sparkleshare.org/support",
                Some("sparkleshare.org/support"),
            )
        );
        println!();
        println!("Options:");
        println!("    --help, --version, --deps, --env");
        println!();
    }
}


impl App {
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

    fn cli_command_clone_usage() {
        println!("Usage: twinkle clone <user@host:path> [path]");
        println!("               clone <ssh://user@host[:port]/path> [path]");
        println!();
    }


    pub fn cli_command_init(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>>{
        self.cli_require_args(2, args).map_err(|_| {
            Self::cli_command_clone_usage();
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
        println!("     {} {}", cli_dimmed("Remote:"), repo.remote_url().unwrap());
        println!("     {} {}", cli_dimmed("Branch:"), repo.branch().unwrap_or("err".to_string()));
        println!();
        println!("    {} {}", cli_dimmed("Enabled:"), repo.enabled());
        println!("         {} {}", cli_dimmed("ID:"), repo.id().unwrap());
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


impl App {
    /// Checks if the minimum amount of args have been passed
    fn cli_require_args(&self, count: usize, args: &[String]) -> Result<(), Box<dyn Error>> {
        if args.len() - 1 < count {
            self.cli_option_help();
            return Err(format!("Command requires {count} arguments").into());
        }

        Ok(())
    }

    /// Finds the toplevel Git repository and returns the absolute path
    fn cli_prepare_path(&self, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let path = fs::canonicalize(path)?;
        GitEnvironment::new(&path).rev_parse_show_toplevel()
    }
}


// Docs: https://jvns.ca/blog/2025/03/07/escape-code-standards/
pub fn cli_bold(s: &str) -> String {   format!("\x1b[1m{}\x1b[0m",  s) }
pub fn cli_dimmed(s: &str) -> String { format!("\x1b[2m{}\x1b[0m",  s) }
pub fn cli_error(s: &str) -> String {  format!("\x1b[31m{}\x1b[0m", s) }

pub fn cli_link(url: &str, label: Option<&str>) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, label.unwrap_or(url))
}
