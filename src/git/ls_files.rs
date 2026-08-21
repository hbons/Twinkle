//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-ls-files

    pub fn ls_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let output = self.run("ls-files", &[
            OsStr::new("-z"),
        ])?;

        Ok(output_to_paths(&output.stdout))
    }


    pub fn ls_files_ignored(&self) -> Result<Vec<PathBuf>, Box<dyn Error>>{
        let output = self.run("ls-files", &[
            OsStr::new("--ignored"),
            OsStr::new("--deduplicate"),
            OsStr::new("--directory"), // Don't recurse into ignored directories, just list once
            OsStr::new("--exclude-standard"), // Use .git/info/exclude, .gitignore files, and the global gitignore
            OsStr::new("--others"), // Show untracked files
            OsStr::new("-z"),
        ])?;

        Ok(output_to_paths(&output.stdout))
    }


    pub fn ls_files_killed(&self) -> Result<Vec<PathBuf>, Box<dyn Error>>{
        let output = self.run("ls-files", &[
            OsStr::new("--killed"), // Lists problematic untracked paths that block a proper checkout
            OsStr::new("-z"),
        ])?;

        Ok(output_to_paths(&output.stdout))
    }
}


fn output_to_paths(output: &Vec<u8>) -> Vec<PathBuf> {
    output
        .split(|&b| b == 0) // NUL-byte
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(OsStr::from_bytes(path)))
        .collect()
}
