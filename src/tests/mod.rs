//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::unwrap_used)]

#[cfg(test)]
mod ssh {
    mod test_ssh_keygen;
    mod test_ssh_keyscan;
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
}

#[cfg(test)]
mod twinkle {
    mod test_twinkle_commit_message;
    mod test_twinkle_config;
    mod test_twinkle_default_dir_name;
    mod test_twinkle_default;
    mod test_twinkle_email_is_valid;
    mod test_twinkle_pretty_bool;
    mod test_twinkle_pretty_datetime;
    mod test_twinkle_pretty_dir;
    mod test_twinkle_resolve_paths;
    mod test_twinkle_unique_dir;
    mod test_twinkle_settings_url_for_host;
    mod test_twinkle_watch;
}
