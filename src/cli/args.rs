//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::env;
use std::error::Error;

use crate::runtime::{ self, Runtime };
use crate::git::objects::environment::GitEnvironment;

use super::util::*;


impl Runtime {
    pub async fn cli_parse_args(
        &mut self,
        args: &[String],
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(1, args)?;

        let command = args.get(1).ok_or("Missing <command>")?;

        match command.as_str() {
            "clone"     => self.cli_command_clone(args)?,
            "init"      => self.cli_command_init(args)?,
            "sync"      => self.cli_command_sync(args).await?,
            "status"    => self.cli_command_status(args)?, // Not displayed
            "check"     => self.cli_command_check(args)?, // Not displayed
            "--help"    => self.cli_option_help(),
            "--version" => self.cli_option_version(),
            "--deps"    => self.cli_option_deps(),
            "--env"     => self.cli_option_env()?,
            _ => {
                self.cli_option_help();
                return Err("Unknown command".into());
            }
        }

        Ok(())
    }


    pub fn cli_option_help(&self) {
        println!("Usage: {} <command> [args…]", self.command);
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

impl Runtime {
    pub fn cli_option_version(&self) {
        println!("{}", runtime::version());
    }


    pub fn cli_option_deps(&self) {
        println!("{}", runtime::deps());
    }


    pub fn cli_option_env(&self) -> Result<(), Box<dyn Error>>{
        println!("{:#?}", self);

        let git = GitEnvironment::new(
            &env::current_dir()?
        );

        if let Ok(output) = git.config_list() {
            println!("{}", lossy_and_trim(&output.stdout));
        }

        Ok(())
    }
}
