//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-ls-remote

    pub fn ls_remote(&self, branch: &str) -> Result<String, Box<dyn Error>> {
        let output = self.run("ls-remote", &[
            "--exit-code", // Use exit codes on errors
            "--heads", // '--branches' after Git 2.46.0 (Sep 11 2024)
            "--quiet", // Don't print remote to stderr
            "origin",
            branch,
        ])?;

        match output.exit_code {
            0   => (), // Successful connection
            2   => return Err("No matching remote branch".into()),
            128 => return Err("No connection".into()),
            _   => return Err("Unknown error".into()),
        }

        // '950264636c68591989456e3ba0a5442f93152c1a	refs/heads/main'
        output.stdout.split('\t').next()
            .map(|remote_id| remote_id.to_string())
            .ok_or_else(|| "Cannot parse remote id".into())
    }
}
