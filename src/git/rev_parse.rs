//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{ Path, PathBuf };

use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-rev-parse

    pub fn rev_parse(&self) -> Result<String, Box<dyn Error>> {  // TODO: GitID
        let rev_parse = self.run("rev-parse", &[
            OsStr::new("--verify"),
            OsStr::new("HEAD"),
        ]);

        match rev_parse {
            Err(_) => Err("No commits yet".into()), // FIXME: non-git dirs also error...
            Ok(output) => Ok(Self::lossy_and_trim(&output.stdout)),
        }
    }


    pub fn rev_parse_show_toplevel(&self) -> Result<PathBuf, Box<dyn Error>> {
        let rev_parse = self.run("rev-parse", &[
            OsStr::new("--show-toplevel"),
        ]);

        match rev_parse {
            Err(_) => Err("Not a Git repository".into()),
            Ok(output) => {
                let s = OsStr::from_bytes(output.stdout.trim_ascii_end());
                let path = Path::new(s).to_path_buf();

                Ok(path)
            },
        }
    }
}
