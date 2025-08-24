//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };

use crate::git::objects::change::GitChange;
use crate::git::objects::file_status::GitFileStatus;

use crate::ssh::objects::config::SshConfig;
use crate::ssh::objects::url::SshUrl;
use crate::ssh::keys::key_pair::KeyPair;


// See: https://youtu.be/xxX81WmXjPg
pub fn twinkle_email_is_valid(email: &str) -> bool {
    email.contains('@') &&
    email.len() <= 254
}


// "ssh://git@github.com:hbons/Twinkle" -> "Twinkle"
// "ssh://git@github.com:hbons"         -> "hbons"
pub fn twinkle_default_dir_name(url: &SshUrl) -> Result<PathBuf, Box<dyn Error>> {
    let dir = url.path.file_stem().ok_or("Could not determine path")?;
    Ok(PathBuf::from(dir))
}


// "Projects/Folder" exists?     -> "Projects/Folder (2)"
// "Projects/Folder (2)" exists? -> "Projects/Folder (3)" etc.
pub fn twinkle_unique_dir(dir: &Path) -> PathBuf {
    let mut unique_dir = dir.to_path_buf();
    let mut suffix = 2;

    while unique_dir.exists() {
        let path = format!("{} {suffix}", dir.display());
        unique_dir = Path::new(&path).to_path_buf();
        suffix += 1;
    }

    unique_dir.to_path_buf()
}


// '+10, ~7, -3'
// '~ "README.md"'
pub fn twinkle_commit_message(status: &Vec<GitChange>) -> Option<String> {
    let (mut added, mut modified, mut deleted) = (0, 0, 0);
    let mut file = String::new();

    for change in status {
        match change.status_x {
            Some(GitFileStatus::Added)       => { added += 1; },
            Some(GitFileStatus::Modified)    => { modified += 1; },
            Some(GitFileStatus::Deleted)     => { deleted += 1; },
            Some(GitFileStatus::Renamed(_))  => { deleted += 1; added += 1; },
            Some(GitFileStatus::Copied(_))   => { added += 1; },
            _ => ()
        };

        file = change.path.to_string_lossy().to_string();
    }

    match added + modified + deleted {
        0 => None,
        1 if added    == 1 => Some(format!("+ \"{file}\"")),
        1 if modified == 1 => Some(format!("~ \"{file}\"")),
        1 if deleted  == 1 => Some(format!("− \"{file}\"")),
        _ => {
            let mut message = Vec::new();
            if added    > 0 { message.push(format!("+{added}")); }
            if modified > 0 { message.push(format!("~{modified}")); }
            if deleted  > 0 { message.push(format!("−{deleted}")); }

            Some(message.join(", "))
        }
    }
}


pub fn twinkle_settings_url_for(host: String) -> Option<&'static str> {
    match host.as_str() {
        "bitbucket.org"    => Some("https://bitbucket.org/account/settings/ssh-keys/"),
        "codeberg.org"     => Some("https://codeberg.org/user/settings/keys"),
        "github.com"       => Some("https://github.com/settings/keys"),
        "gitlab.com"       => Some("https://gitlab.com/-/user_settings/ssh_keys"),
        "gitlab.gnome.org" => Some("https://gitlab.gnome.org/-/user_settings/ssh_keys"),
        "git.sr.ht"        => Some("https://meta.sr.ht/keys/ssh-keys"),
        _ => None
    }
}


pub fn twinkle_ssh_command(key_pair: &KeyPair) -> String {
    let config = SshConfig {
        IdentityFile: key_pair.private_key_path.clone(),
        UserKnownHostsFile: key_pair.private_key_path.with_extension("key.host"),
        ..Default::default()
    };

    format!("ssh -F /dev/null {config}")
}
