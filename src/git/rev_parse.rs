//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::path::{ Path, PathBuf };
use std::os::unix::ffi::OsStrExt;

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-rev-parse

    pub fn rev_parse(&self) -> Result<String, Box<dyn Error>> {  // TODO: GitID
        let rev_parse = self.run("rev-parse", &[
            OsStr::new("--verify"),
            OsStr::new("HEAD"),
        ]);

        match rev_parse {
            Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).into()),
            Err(_) => Err("No commits yet".into()), // FIXME: non-git dirs also error...
        }
    }


    pub fn rev_parse_show_toplevel(&self) -> Result<PathBuf, Box<dyn Error>> {
        match self.run("rev-parse", &[OsStr::new("--show-toplevel")]) {
            Ok(output) => {
                let path = OsStr::from_bytes(&output.stdout);
                let path = Path::new(path).to_path_buf();

                Ok(path)
            },
            Err(_) => Err("No commits yet".into()),
        }
    }
}
