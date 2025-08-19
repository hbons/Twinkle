//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::PathBuf;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-ls-files

    pub fn ls_files(&self) -> Result<String, Box<dyn Error>> {
        let output = self.run("ls-files", &[])?;
        Ok(output.stdout)
    }


    pub fn ls_files_ignored(&self) -> Result<Vec<PathBuf>, Box<dyn Error>>{
        let output = self.run("ls-files", &[
            "--ignored",
            "--directory", // Don't recurse into ignored directories, just list once
            "--exclude-standard", // Use .git/info/exclude, .gitignore files, and the global gitignore
            "--others" // Show untracked files
        ])?;

        let files = output.stdout.trim().split("\n");
        let files = files.map(|f| PathBuf::from(f)).collect();

        Ok(files)
    }
}
