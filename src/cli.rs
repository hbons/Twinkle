//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::io::{ self, Write };
use std::path::{ Path, PathBuf };
use std::thread;
use std::time::Duration;

use crate::app::{ App, app_deps, app_version };
use crate::git::objects::environment::GitEnvironment;
use crate::log;

use crate::ssh::objects::url::SshUrl;
use crate::ssh::keys::key_type::KeyType;

use crate::twinkle::twinkle_clone::TwinkleCloneError;
use crate::twinkle::twinkle_clone::twinkle_clone_prepare;
use crate::twinkle::twinkle_clone::twinkle_clone_start;
use crate::twinkle::twinkle_clone::twinkle_clone_complete;

use crate::twinkle::twinkle_default::twinkle_default_polling_interval;
use crate::twinkle::twinkle_keys::twinkle_hostkey_trust;
use crate::twinkle::twinkle_keys::twinkle_keypair_for;
use crate::twinkle::twinkle_pretty::{ twinkle_pretty_bool, twinkle_pretty_datetime, twinkle_pretty_dir };
use crate::twinkle::twinkle_util::twinkle_settings_url_for;
use crate::twinkle::twinkle_watch::twinkle_watch;


impl App {
    pub fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let command = args.get(1).ok_or("Missing <command>")?;

        match command.as_str() {
            "clone"     => self.cli_command_clone(args)?,
            "watch"     => self.cli_command_watch(args)?,
            "status"    => self.cli_command_status(args)?,
            "list"      => self.cli_command_list()?,
            "remove"    => self.cli_command_remove(args)?,
            "config"    => self.cli_command_config(args)?,
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
        println!("Usage: twinkle <command> [arguments…]");
        println!();
        println!("Commands:");
        println!("    clone  <user@host:path> <path>");
        println!("    watch  <path> [--interval=60]");
        println!("    remove <path>");
        println!("    status <path>");
        println!("    list");
        println!();
        println!("Configuration:");
        println!("    config <path> <option> <value>");
        println!();
        println!("Options:");
        println!("    --help, --version, --deps, --env");
        println!();
    }
}


impl App {
    pub fn cli_command_clone(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>>{
        self.cli_require_args(3, args)?;

        let ssh_url = args.get(2).ok_or("Missing <user@host:path>")?;
        let path = Path::new(args.get(3).ok_or("Missing <path>")?);

        let ssh_url = ssh_url.parse::<SshUrl>().map_err(|_| {
            Self::cli_command_clone_usage();
            "Not a valid <user@host:path>"
        })?;

        let path = self.cli_prepare_path(path).map_err(|_| {
            Self::cli_command_clone_usage();
            "Not a valid <path>"
        })?;

        loop {
            match twinkle_clone_prepare(&ssh_url, &self.app_keys_dir) {
                Err(e) => match e.downcast_ref::<TwinkleCloneError>() {
                    Some(TwinkleCloneError::NeedsNetwork) => {
                        println!("Could not connect to {}", ssh_url.host);
                    },
                    Some(TwinkleCloneError::NeedsTrust(host_key)) => {
                        let fingerprint = host_key.fingerprint.clone().unwrap_or_else(|| {
                            log::error_and_exit("No host key fingerprint");
                        });

                        println!("Connected to {} {} {} ",
                            host_key.host, cli_dimmed("–"), cli_dimmed(&fingerprint.to_string()));

                        io::stdout().flush()?;
                        twinkle_hostkey_trust(host_key, &self.app_keys_dir)?;
                    },
                    Some(TwinkleCloneError::NeedsAuth(host_key, key_pair)) => {
                        let url = twinkle_settings_url_for(host_key.host.clone());
                        let url = url.unwrap_or(&host_key.host);
                        let settings_url = cli_link(url, None);

                        println!();
                        println!("First, add this SSH key to {settings_url}:");
                        println!("{}", cli_bold(&key_pair.public_key));
                        println!();
                        print!("Then, press {enter} to clone {url}… ",
                            enter=cli_bold("[Enter]"),
                            url=cli_bold(&ssh_url.original));

                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                    },
                    None => return Err(e),
                },
                Ok(key_pair) => {
                    let mut repo = twinkle_clone_start(&ssh_url, &key_pair, &path)?;
                    twinkle_clone_complete(&mut repo, &key_pair)?;
                    self.config.add(&repo)?;

                    return Ok(());
                }
            };

            thread::sleep(Duration::from_millis(500));
        }
    }

    fn cli_command_clone_usage() {
        println!("Usage: twinkle clone <user@host:path> <path>");
        println!("               clone <ssh://user@host[:port]/path> <path>");
        println!();
    }


    pub fn cli_command_watch(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(2, args)?; // TODO: Get --interval=n

        let path = Path::new(args.get(2).ok_or("Missing <path>")?);
        let path = self.cli_prepare_path(path)?;

        let repo = self.config.find(&path)?;
        repo.git.working_dir = repo.path.to_path_buf();

        let dir = twinkle_pretty_dir(&repo.path);

        let remote = cli_dimmed(&format!("– {}…\n", repo.remote_url.original));
        log::log(&format!("Watching {} {}", cli_bold(&dir), remote));

        repo.user.key_pair = twinkle_keypair_for(&repo.remote_url.host,
            KeyType::default(), &self.app_keys_dir).ok();

        twinkle_watch(repo)?;
        Ok(())
    }


    pub fn cli_command_status(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>>{
        self.cli_require_args(2, args)?;

        let path = Path::new(args.get(2).ok_or("Missing <path>")?);
        let path = self.cli_prepare_path(path)?;

        let repo = self.config.find(&path)?;
        let path = twinkle_pretty_dir(&path);

        let interval = repo.polling_interval.unwrap_or(
            twinkle_default_polling_interval()
        );

        println!();
        println!("       {} {}", cli_dimmed("Path:"), cli_bold(&path));
        println!("     {} {}", cli_dimmed("Remote:"), repo.remote_url.to_string_alternate());
        println!("       {} {}", cli_dimmed("User:"), repo.user);
        println!("     {} {}", cli_dimmed("Branch:"), repo.branch);
        println!("        {} {}", cli_dimmed("LFS:"), twinkle_pretty_bool(repo.lfs));
        println!();
        println!(" {} {}", cli_dimmed("Last check:"), twinkle_pretty_datetime(repo.last_checked));
        println!("  {} {}", cli_dimmed("Last sync:"), twinkle_pretty_datetime(repo.last_synced));
        println!("   {} {}s", cli_dimmed("Interval:"), interval);
        println!();

        Ok(())
    }


    pub fn cli_command_list(&self) -> Result<(), Box<dyn Error>>{
        println!();

        for repo in self.config.list()? {
            let path = twinkle_pretty_dir(&repo.path);
            let url = repo.remote_url.to_string_with_port();

            println!("       {} {}", cli_dimmed("Path:"), cli_bold(&path));
            println!("     {} {}", cli_dimmed("Remote:"), url);
            println!();
        }

        Ok(())
    }


    pub fn cli_command_remove(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(2, args)?;

        // Not using cli_prepare_path() to prevent mistakes
        let path = Path::new(args.get(2).ok_or("Missing <path>")?);
        let path = fs::canonicalize(path)?;

        self.config.remove(&path)?;
        Ok(())
    }


    pub fn cli_command_config(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(4, args)?;

        let path = Path::new(args.get(2).ok_or("Missing <path>")?);
        let option = args.get(3).ok_or("Missing <option>")?;
        let value  = args.get(4).ok_or("Missing <value>")?;

        let path = self.cli_prepare_path(path)?;

        match option.as_str() {
            "interval" => {
                let value = value.parse::<u64>().map_err(|_| "Interval must be a number")?;
                self.config.set_interval(&path, value)?;
            },
            "lfs" => {
                let value = value.as_str() == "true";
                self.config.set_lfs(&path, value)?;
            }
            "user.name"  => { self.config.set_user(&path, Some(value), None)?; },
            "user.email" => { self.config.set_user(&path, None, Some(value))?; },
            _ => { return Err(format!("Unknown option `{}`", option).into()); }
        }

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
