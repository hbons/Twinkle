//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::PathBuf;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-rev-parse

    pub fn rev_parse(&self) -> Result<String, Box<dyn Error>> {
        match self.run("rev-parse", &["--verify", "HEAD"]) {
            Ok(output) => Ok(output.stdout),
            Err(_) => Err("No commits yet".into()),
        }
    }


    pub fn rev_parse_show_toplevel(&self) -> Result<PathBuf, Box<dyn Error>> {
        match self.run("rev-parse", &["--show-toplevel"]) {
            Ok(output) => Ok(PathBuf::from(output.stdout.to_string())),
            Err(_) => Err("No commits yet".into()),
        }
    }


    pub fn is_repo_empty(&self) -> bool {
        self.rev_parse().is_err()
    }
}
