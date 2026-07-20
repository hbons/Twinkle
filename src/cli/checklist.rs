//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::app::App;
use super::util::*;


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


    fn run_checklist(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        println!();
        println!("  Platform:");
        println!();
        self.run_check("Supported OS: ", &is_git_installed, &path);
        println!();

        println!("  Dependencies:");
        println!();
        self.run_check("OpenSSH found: 2.4.34", &is_git_installed, &path);
        self.run_check("Git found: ", &is_git_installed, &path);
        self.run_check("Git LFS found: ", &is_git_installed, &path);
        println!();

        println!("  Secure Shell:");
        println!();
        self.run_check("ssh-agent running ", &is_git_installed, &path);
        self.run_check("Keys in agent ", &is_git_installed, &path);
        println!();

        println!("  Connectivity:");
        println!();
        self.run_check("Network connection", &is_git_installed, &path);
        self.run_check("Host reachable", &is_git_installed, &path);
        self.run_check("Host uses SSH", &is_git_installed, &path);
        self.run_check("Host supports ED25519", &is_git_installed, &path);
        self.run_check("Host supports ECDSA", &is_git_installed, &path);
        self.run_check("Host supports RSA", &is_git_installed, &path);
        self.run_check("Host knows local SSH key", &is_git_installed, &path);
        println!();

        println!("  Repository:");
        println!();
        self.run_check(".git/ exists", &is_git_installed, &path);
        self.run_check(".git/config valid", &is_git_installed, &path);
        self.run_check(".git/config/exclude valid", &is_git_installed, &path);
        self.run_check(".git/config/attributes valid", &is_git_installed, &path);
        self.run_check("On a branch", &is_git_installed, &path);
        self.run_check("Remote origin URL valid", &is_git_installed, &path);
        self.run_check("Files treated as binary", &is_git_installed, &path);
        self.run_check("User name set", &is_git_installed, &path);
        self.run_check("User email set", &is_git_installed, &path);
        self.run_check("User signing key set", &is_git_installed, &path);
        self.run_check("Commit signing enabled", &is_git_installed, &path);
        // check important git settings
        println!();

        println!("  Twinkle:"); // bold
        println!();
        self.run_check("Enabled", &is_git_installed, &path);
        self.run_check(".twinkle/config valid", &is_git_installed, &path);
        self.run_check("Push notifications enabled", &is_git_installed, &path);
        self.run_check("Git LFS enabled", &is_git_installed, &path);
        self.run_check("Git LFS size threshold set", &is_git_installed, &path);

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
        let bar = cli_dimmed("");

        println!("    {bar}\x1b[32m{}\x1b[0m{bar} {s}", &cli_bold(&result.unwrap().to_string()).to_string());

    }
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
            Self::Fail => write!(f, "✓"),
        }
    }
}


pub fn is_git_installed(_path: &Path) -> Result<CheckStatus, Box<dyn Error>> {
    // let git = GitEnvironment::new(path);
    // git.branch()?
    Ok(CheckStatus::Fail)

}
