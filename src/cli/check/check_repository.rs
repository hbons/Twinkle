//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
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


pub fn is_git_info_exclude_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/info/exclude").exists() {
        Ok(Check::Pass(None)) // TODO: parse file
    } else {
        Ok(Check::Fail(None))
    }
}

pub fn is_git_info_attributes_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/info/attributes").exists() {
        Ok(Check::Pass(None)) // TODO: parse file
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_on_a_branch(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if git.symbolic_ref().is_err() {
        return Ok(Check::Fail(None))
    }

    let branch = git.branch_show_current()?;

    Ok(Check::Pass(Some(branch)))
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
    let path = path.join(".git/info/attributes");

    if path.exists() {
        let content = fs::read_to_string(path)?;

        if content.contains("* merge=binary") {
            return Ok(Check::Pass(None));
        }
    }

    Ok(Check::Fail(None))
}
