//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::{error::Error, ffi::OsStr};
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-symbolic-ref

    pub fn symbolic_ref(&self) -> Result<String, Box<dyn Error>> {
        let symbolic_ref = self.run("symbolic-ref", &[
            OsStr::new("--quiet"),
            OsStr::new("HEAD")
        ]);

        match symbolic_ref {
            Ok(output) => Ok(output.stdout),
            Err(_) => Err("Detached HEAD".into()), // Status code `1` on detached HEADs
        }
    }
}
