//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::{ Path, PathBuf };

use crate::app::App;
use crate::git::objects::environment::GitEnvironment;


impl App {
    /// Checks if the minimum amount of args have been passed
    pub fn cli_require_args(&self, count: usize, args: &[String]) -> Result<(), Box<dyn Error>> {
        if args.len() - 1 < count {
            self.cli_option_help();
            return Err(format!("Command requires {count} arguments").into());
        }

        Ok(())
    }

    /// Finds the toplevel Git repository and returns the absolute path
    pub fn cli_prepare_path(&self, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
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

pub fn cli_green(s: &str) -> String {
    cli_bold(
        &format!("\x1b[32m{}\x1b[0m", s)
    )
}

pub fn cli_yellow(s: &str) -> String {
    cli_bold(
        &format!("\x1b[33m{}\x1b[0m", s)
    )
}

pub fn cli_red(s: &str) -> String {
    cli_bold(
        &format!("\x1b[31m{}\x1b[0m", s)
    )
}
