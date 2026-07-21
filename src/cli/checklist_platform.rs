//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env::{ consts::ARCH, consts::OS };
use std::path::Path;
use std::process::Command;

use crate::git::objects::environment::GitEnvironment;

use super::*;
use super::checklist_ssh::*;
use crate::cli::checklist::Check;


// Platform

pub fn is_supported_os(_path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(
        match OS {
            "linux" |
            "macos" => Check::Pass(Some(OS.into())),
            s if s.ends_with("bsd") => Check::Pass(Some(OS.into())),
            _ => Check::Fail(Some(OS.into())),
        }
    )
}

pub fn is_supported_arch(_path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(
        match ARCH {
            s if s.starts_with("x86") => Check::Pass(Some(ARCH.into())),
            "arm" | "aarch64" => Check::Pass(Some(ARCH.into())),
            _ => Check::Fail(Some(ARCH.into())),
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
