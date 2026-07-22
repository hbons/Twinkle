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
            return Ok(Check::Pass(Some(output.stdout)));
        }
    }

    Ok(Check::Missing)
}


pub fn is_git_user_email_set(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("user.email") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(Some(output.stdout)));
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


pub fn is_git_commit_signing_enabled(path: &Path) -> Result<Check, Box<dyn Error>> {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get("commit.gpgSign") {
        if output.exit_code == 0 {
            return Ok(Check::Pass(None));
        }
    }

    Ok(Check::Missing)
}



// Sync Config

// run_check("Enabled", &is_twinkle_enabled, &path);
// run_check(".twinkle/config valid", &is_twinkle_config_valid, &path);
// run_check("Git LFS enabled", &is_git_lfs_enabled, &path);
// run_check("Git LFS size threshold set", &is_git_lfs_threshold_set, &path);
// run_check("Push notifications enabled", &is_twinkle_push_noticications_enabled, &path);
// run_check("Push notifications URL set", &is_twinkle_push_noticications_url_set, &path);



// TODO: find all git config options/filters/hooks that may have been added by the user
