//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use super::objects::change::GitChange;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-status

    pub fn status(&self) -> Result<Vec<GitChange>, Box<dyn Error>> {
        // Show untracked files/dirs
        let changes = self.get_changes("--untracked-files=normal")?;
        // let filtered_changes = changes.iter().filter(|change| change.status == GitFileStatus::Untracked).collect::<Vec<_>>();
        Ok(changes)
    }


    fn get_changes(&self, extra_arg: &str) -> Result<Vec<GitChange>, Box<dyn Error>> {
        let output = &self.run("status", &[
            "--no-renames",
            "--porcelain",
            extra_arg
        ])?;

        let mut changes = Vec::new();

        for line in output.stdout.lines() {
            let change = line.parse::<GitChange>()?;
            changes.push(change);
        }

        Ok(changes)
    }
}
