//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::app::App;
// use super::util::*;


impl App {
    pub fn cli_command_checklist(
        &mut self,
        args: &Vec<String>,
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        self.run_checklist(&path)
    }



fn run_checklist(&self, path: &Path)
-> Result<(), Box<dyn Error>>
{
    println!();
    println!("  Platform:");
    println!();

    println!();
    println!("  Dependencies:");
    println!();
    self.run_check("OpenSSH found: 2.4.34", &is_git_installed, &path);
    self.run_check("Git found: ", &is_git_installed, &path);
    self.run_check("Git LFS found: ", &is_git_installed, &path);
    println!();

    println!("  SSH:");
    println!();
    // check keys present
    // check ssh-agent running
    // check ssh-agent has keys
    // check host reachable ping host. nc?
    println!();

    println!("  Connectivity");
    println!();
    self.run_check("Internet connection: ", &is_git_installed, &path);
    self.run_check("ping host: ", &is_git_installed, &path);
    self.run_check("SSH to host: ", &is_git_installed, &path);
    self.run_check("SSH auth to host: ", &is_git_installed, &path);
    println!();

    println!("  Repository:");
    println!();
    // check path exists
    // check .git present OK
    // check .git/config valid OK (green)
    // check on a branch
    // check remote.origin.url
    // check user name set
    // check user email set
    // check user signing key set
    // check commit signing enabled
    // check important git settings
    println!();

    println!("  Twinkle:");
    println!();
    // check twinkle.enabled MISSING (red)
    // check twinkle.lfs.enabled | NOT SET (yellow)
    // check twinkle.lfs.sizeThreshold | NOT SET (yellow)
    // check .git/info/exclude: n
    // check .git/info/attributes filter
    println!();

    // twinkle config
    // check .twinkle/config valid
    // print all twinkle vars. if missing: DEFAULT (green)
    //
    // git config --list --show-origin

    Ok(())
}

    fn run_check(&self,
        s: &str,
        f: &dyn Fn(&Path)  -> Result<CheckStatus, Box<dyn Error>>,
        path: &Path, // TODO: use current_dir?
    ) {
        let result = f(path);

        // TODO: if has output, append ": {output}"
        println!("    {} {s}", result.unwrap());

    }
}




pub fn is_git_installed(path: &Path) -> Result<CheckStatus, Box<dyn Error>> {
    // let git = GitEnvironment::new(path);
    // git.branch()?
    Ok(CheckStatus::Fail)

}

pub enum CheckStatus {
    Pass,
    Default,
    Missing, // optional setting
    Fail,
}

impl fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "✓"),
            Self::Default => write!(f, "✓"),
            Self::Missing => write!(f, "✓"),
            Self::Fail => write!(f, "X"),
        }
    }
}
