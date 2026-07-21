//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use super::checklist::Check;


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
    if path.join(".git/MERGE_HEAD").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/REVERT_HEAD").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/BISECT_LOG").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/BISECT_START").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/CHERRY_PICK_HEAD").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/rebase_merge/").exists() {
        return Ok(Check::Fail(None))
    }

    if path.join(".git/rebase_apply/").exists() {
        return Ok(Check::Fail(None))
    }

    Ok(Check::Pass(None))
}


pub fn is_git_remote_url_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("remote.origin.url") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(None)); // TODO: check URL validity
        }
    }

    Ok(Check::Fail(None))
}


pub fn is_git_user_name_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("user.name") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(None));
        }
    }

    Ok(Check::Missing)
}


pub fn is_git_user_email_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("user.email") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(None));
        }
    }

    Ok(Check::Missing)
}


pub fn is_git_user_signing_key_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("user.signingKey") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(None));
        }
    }

    Ok(Check::Missing)
}
