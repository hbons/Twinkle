//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fs;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use super::outcome::Outcome;


// Repository

pub fn is_git_dir_present(path: &Path) -> Outcome {
    if path.join(".git").exists() {
        Outcome::Pass(None)
    } else {
        Outcome::Fail(None)
    }
}


pub fn is_git_info_exclude_exists(path: &Path) -> Outcome {
    if path.join(".git/info/exclude").exists() {
        Outcome::Pass(None)
    } else {
        Outcome::Fail(None)
    }
}

pub fn is_git_info_attributes_exists(path: &Path) -> Outcome {
    if path.join(".git/info/attributes").exists() {
        Outcome::Pass(None)
    } else {
        Outcome::Fail(None)
    }
}


pub fn is_git_on_a_branch(path: &Path) -> Outcome {
    let git = GitEnvironment::new(path);

    if git.symbolic_ref().is_err() {
        return Outcome::Fail(None)
    }

    let branch = git.branch_show_current();

    match branch {
        Ok(branch) => Outcome::Pass(Some(branch)),
        Err(_) => Outcome::Fail(None),
    }
}


pub fn is_git_not_in_a_merge(path: &Path) -> Outcome {
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
            return Outcome::Fail(None);
        }
    }

    Outcome::Pass(None)
}


pub fn is_git_attributes_all_binary(path: &Path) -> Outcome {
    let path = path.join(".git/info/attributes");

    if path.exists() {
        let content = fs::read_to_string(path);

        match content {
            Ok(c) => {
                if c.contains("* merge=binary") {
                    return Outcome::Pass(None);
                }
            },
            Err(_) => return Outcome::Error,
        }
    }

    Outcome::Fail(Some("missing \"* merge=binary\"".into()))
}
