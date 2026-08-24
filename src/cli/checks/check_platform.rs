//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::env::consts::{ ARCH, OS };
use std::path::Path;
use std::process::Command;

use crate::cli::util::lossy_and_trim;
use crate::git::objects::environment::GitEnvironment;
use super::outcome::Outcome;


// Platform

pub fn is_supported_os(_path: &Path) -> Outcome {
    let os = Some(OS.into());

    match OS {
        "linux" | "macos" => Outcome::Pass(os),
        s if s.ends_with("bsd") => Outcome::Pass(os),
        _ => Outcome::Fail(os),
    }
}

pub fn is_supported_arch(_path: &Path) -> Outcome {
    let arch = Some(ARCH.into());

    match ARCH {
        "x86_64"  => Outcome::Pass(arch),
        "aarch64" => Outcome::Pass(arch),
        _ => Outcome::Fail(arch),
    }
}

pub fn is_supported_runtime(_path: &Path) -> Outcome {
    match env::var("FLATPAK_ID") {
        Ok(s) =>  Outcome::Pass(Some(format!("flatpak ({s})"))),
        Err(_) => Outcome::Pass(Some("native".into())),
    }
}


// Dependencies

pub fn is_openssh_installed(_path: &Path) -> Outcome {
    let ssh = Command::new("ssh")
        .arg("-V")
        .output();

    match ssh {
        Ok(output) => Outcome::Pass(
            Some(lossy_and_trim(&output.stderr))
        ),
        _ => Outcome::Missing,
    }
}

pub fn is_git_installed(path: &Path) -> Outcome {
    match GitEnvironment::new(path).version {
        Some(s) => Outcome::Pass(Some(s.to_string())),
        _ => Outcome::Missing,
    }
}

pub fn is_git_lfs_installed(path: &Path) -> Outcome {
    match GitEnvironment::new(path).lfs_version() {
        Some(s) => Outcome::Pass(Some(s)),
        _ => Outcome::Missing,
    }
}
