//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use crate::app::{ App, app_deps, app_version };
use super::util::*;


impl App {
    pub fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let command = args.get(1).ok_or("Missing <command>")?;

        match command.as_str() {
            "clone"     => self.cli_command_clone(args)?,
            "init"      => self.cli_command_init(args)?,
            "sync"      => self.cli_command_sync(args)?,
            "status"    => self.cli_command_status(args)?, // Not displayed
            "check"     => self.cli_command_checklist(args)?, // Not displayed
            "--help"    => self.cli_option_help(),
            "--version" => println!("{}", app_version()),
            "--deps"    => println!("{}", app_deps()),
            "--env"     => println!("{:#?}", self), // TODO: git config --list --show-origin

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
        println!("    check [path]");
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
