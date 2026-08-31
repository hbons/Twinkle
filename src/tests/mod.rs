//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::unwrap_used)]


#[cfg(test)]
mod core {
    mod test_default_dir_name; // TODO: test_core_?
    mod test_default;
    mod test_object_repository;
    mod test_pretty_bool;
    mod test_pretty_commit_message;
    mod test_pretty_datetime;
    mod test_pretty_dir;
    mod test_resolve_paths;
    mod test_unique_dir;
    mod test_settings_url_for_host;
    mod test_sync;
}

#[cfg(test)]
mod ssh {
    mod test_ssh_keygen;
    mod test_ssh_keyscan;
    mod test_ssh_known_hosts;
    mod test_ssh_object_hostkey;
    mod test_ssh_object_keytype;
    mod test_ssh_object_url;
    mod test_ssh_version;
}

#[cfg(test)]
mod git {
    mod test_git_object_change;
    mod test_git_object_commit_message;
    mod test_git_object_environment;
    mod test_git_object_file_status;
    mod test_git_object_merge_status;
    mod test_git_object_user;
    mod test_git_object_version;
}
