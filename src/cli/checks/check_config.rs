//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::Path;

use crate::git::objects::environment::GitEnvironment;
use super::outcome::Outcome;


fn get_from_config(path: &Path, name: &str, expect: Option<&str>) -> Outcome {
    let git = GitEnvironment::new(path);

    if let Ok(output) = git.config_get(name) {
        let stdout = output.stdout;

        if output.exit_code != 0 {
            return Outcome::Fail(Some(stdout));
        }

        if expect.is_none() {
            return Outcome::Pass(Some(stdout));
        }

        if expect == Some(stdout.as_str().trim()) {
            if expect == Some("") {
                return Outcome::Pass(Some("\"\"".into()));
            }

            return Outcome::Pass(Some(stdout));
        } else {
            return Outcome::Fail(Some(stdout));
        }
    }

    Outcome::Missing
}


// Git

pub fn is_git_config_valid(path: &Path) -> Outcome {
    let output = GitEnvironment::new(path)
        .run("config", &["--list"]);

    match output {
        Ok(o) if o.exit_code == 0 => return Outcome::Pass(None),
        _ => return Outcome::Fail(None),
    }
}

pub fn is_twinkle_config_valid(path: &Path) -> Outcome {
    let config_path = &path.join(
        format!(".{}/config",
            env::var("CARGO_BIN_NAME").unwrap_or("twinkle".into())
        )
    );

    let output = GitEnvironment::new(path)
        .run("config", &["--file", &config_path.to_string_lossy(), "--list"]);

    match output {
        Ok(o) if o.exit_code == 0 => return Outcome::Pass(None),
        _ => return Outcome::Fail(None),
    }
}


pub fn is_git_remote_url_valid(path: &Path) -> Outcome {
    get_from_config(path, "remote.origin.url", None)
}

pub fn is_git_core_attributes_file_set(path: &Path) -> Outcome {
    get_from_config(path, "core.attributesFile", Some(""))
}

pub fn is_git_core_excludes_file_set(path: &Path) -> Outcome {
    get_from_config(path, "core.excludesFile", Some(""))
}

pub fn is_git_submodule_recurse_set(path: &Path) -> Outcome {
    get_from_config(path, "submodule.recurse", Some("false"))
}

pub fn is_git_push_default_set(path: &Path) -> Outcome {
    get_from_config(path, "push.default", Some("current"))
}

pub fn is_git_user_name_set(path: &Path) -> Outcome {
    get_from_config(path, "user.name", None)
}

pub fn is_git_user_email_set(path: &Path) -> Outcome {
    get_from_config(path, "user.email", None)
}


// Sync

pub fn is_twinkle_enabled_set(path: &Path) -> Outcome {
    let check = get_from_config(path, "twinkle.enabled", Some("true"));

    match check {
        Outcome::Missing => Outcome::Fail(Some("missing".into())),
        _ => check,
    }
}

pub fn is_twinkle_id_set(path: &Path) -> Outcome {
    let name = env!("CARGO_BIN_NAME");
    get_from_config(path, &format!("{name}.id"), Some("true"))
}

pub fn is_twinkle_lfs_enabled_set(path: &Path) -> Outcome {
    let name = env!("CARGO_BIN_NAME");
    get_from_config(path, &format!("{name}.lfs.enabled"), Some("true"))
}

pub fn is_twinkle_push_enabled_set(path: &Path) -> Outcome {
    let name = env!("CARGO_BIN_NAME");
    get_from_config(path, &format!("{name}.push.enabled"), Some("true"))
}
