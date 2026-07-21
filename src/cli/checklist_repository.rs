//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

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
