//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use super::objects::environment::GitEnvironment;
use super::objects::output::GitOutput;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-config

    pub fn config_get(&self, name: &str) -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &["--local", name]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["--local", "get", name])
    }

    pub fn config_set(&self, name: &str, value: &str) -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &["--local", name, value]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["--local", "set", name, value])
    }


    pub fn config_file_get(
        &self,
        file: &Path,
        name: &str,
    ) -> Result<GitOutput, Box<dyn Error>>
    {
        let file = file.to_string_lossy().to_string();

        self.run("config", &["--file", &file, name]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["--local", "get", name])
    }

    pub fn config_file_set(
        &self,
        file: &Path,
        name: &str,
        value: &str,
    ) -> Result<GitOutput, Box<dyn Error>>
    {
        let file = file.to_string_lossy().to_string();

        self.run("config", &["--file", &file, name, value]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["--local", "get", name])
    }


    pub fn config_is_valid(&self) -> bool {
        if let Ok(output) = self.run("config", &["--list"]) {
            return output.exit_code == 0
        }

        false
    }
}
