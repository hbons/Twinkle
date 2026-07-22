//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use super::check::Check;


// Git Config

pub fn is_git_config_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".git/config").exists() { // TODO: parse file
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}

pub fn is_twinkle_config_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    if path.join(".twinkle/config").exists() { // TODO: parse file
        Ok(Check::Pass(None))
    } else {
        Ok(Check::Fail(None))
    }
}


pub fn is_git_remote_url_valid(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "remote.origin.url", None)
}

pub fn is_git_core_attributes_file_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "core.attributesFile", Some(&"\"\""))
}

pub fn is_git_core_excludes_file_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "core.excludesFile", Some(&"\"\""))
}

pub fn is_git_ignoring_submodules(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "remote.origin.url", Some(&"false"))
}

pub fn is_git_push_default_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "remote.origin.url", Some(&"current"))
}

pub fn is_git_user_name_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "user.name", None)
}

pub fn is_git_user_email_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "user.email", None)
}

pub fn is_git_user_signing_key_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "user.signingKey", None)
}

pub fn is_git_commit_signing_enabled(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "commit.gpgSign", None)
}


// Sync Config

pub fn is_twinkle_enabled_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "twinkle.enabled", Some(&"true".to_string()))
}

pub fn is_twinkle_lfs_enabled_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "twinkle.lfs.enabled", Some(&"true".to_string()))
}

pub fn is_twinkle_push_enabled_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    get_from_config(path, "twinkle.push.enabled", Some(&"true".to_string()))
}


fn get_from_config(path: &Path, name: &str, expect: Option<&str>) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get(name) {
        if output.exit_code == 0 {
            if Some(output.stdout.as_str()) == expect || expect.is_none() {
                if expect == Some("") {
                    return Ok(Check::Pass(Some("\"\"".into())));
                } else {
                    return Ok(Check::Pass(Some(output.stdout)));
                }
            }
        } else {
            return Ok(Check::Fail(Some(output.stdout)));
        }
    }

    Ok(Check::Missing)
}
