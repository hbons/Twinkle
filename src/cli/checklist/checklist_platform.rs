//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env::consts::{ ARCH, OS };
use std::path::Path;
use std::process::Command;

use crate::git::objects::environment::GitEnvironment;
use super::check::Check;


// Platform

pub fn is_supported_os(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let os = Some(OS.into());

    Ok(
        match OS {
            "linux" | "macos" => Check::Pass(os),
            s if s.ends_with("bsd") => Check::Pass(os),
            _ => Check::Fail(os),
        }
    )
}

pub fn is_supported_arch(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let arch = Some(ARCH.into());

    Ok(
        match ARCH {
            s if s.starts_with("x86") => Check::Pass(arch),
            "arm" | "aarch64" => Check::Pass(arch),
            _ => Check::Fail(arch),
        }
    )
}


// Dependencies

pub fn is_openssh_installed(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let ssh = Command::new("ssh").arg("-V").output();

    Ok(match ssh {
        Ok(o) => Check::Pass(Some(
            String::from_utf8_lossy(&o.stderr).trim().to_string())
        ),
        _ => Check::Missing,
    })
}

pub fn is_git_installed(path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(match GitEnvironment::new(path).version() {
        Some(s) => Check::Pass(Some(s.to_string())),
        _ => Check::Missing,
    })
}

pub fn is_git_lfs_installed(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);
    Ok(Check::Pass(Some(git.lfs_version().unwrap())))
}
