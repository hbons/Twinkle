//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use super::check::Check;


// Repository

pub fn is_git_dir_present(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git").exists() {
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_config_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/config").exists() { // TODO: parse file
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_info_exclude_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/info/exclude").exists() { // TODO: parse file
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_info_attributes_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/info/attributes").exists() { // TODO: parse file
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_on_a_branch(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);
    if git.symbolic_ref().is_err() {
        return Ok(Check::Fail(None))
    }

    Ok(Check::Pass(None))
}


pub fn is_git_not_in_a_merge(path: &Path) -> Result<Check, Box<dyn Error>> {
    let merge_state_files = [
        ".git/MERGE_HEAD",
        ".git/REVERT_HEAD",
        ".git/BISECT_LOG",
        ".git/BISECT_START",
        ".git/CHERRY_PICK_HEAD",
        ".git/rebase_merge/",
        ".git/rebase_apply/",
    ];

    for file in &merge_state_files {
        if path.join(file).exists() {
            return Ok(Check::Fail(None));
        }
    }

    Ok(Check::Pass(None))
}


pub fn is_git_attributes_all_binary(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/info/attributes").exists() { // TODO: parse file
        Ok(Check::Pass(None)) // TODO: check if contains "* merge=binary"
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_ignoring_submodules(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("submodule.recurse") {
        if output.exit_code == 0 &&
        output.stdout.trim() == "false" {
            return Ok(Check::Pass(Some(false.to_string())));
        }
    }

    Ok(Check::Missing)
}
