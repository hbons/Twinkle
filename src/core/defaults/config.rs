//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::thread;


pub const K_ENABLED: &str = "enabled";
pub const K_ID: &str = "id";
pub const K_READONLY: &str = "readonly";
pub const K_POLLING_INTERVAL: &str = "pollingInterval";
pub const K_LAST_SYNC: &str = "lastSync";
pub const K_LAST_CHECK: &str = "lastCheck";
pub const K_MAX_FILE_SIZE: &str = "maxFileSize";

pub const K_CO_AUTHOR: &str = "coAuthor";


// Notify
pub const K_NOTIFY_ENABLED: &str = "notify.enabled";
pub const K_NOTIFY_URL: &str = "notify.url";

// LFS
pub const K_LFS_ENABLED: &str = "lfs.enabled";
pub const K_LFS_SIZE_THRESHOLD: &str = "lfs.sizeThreshold";
pub const K_LFS_MAX_FILE_SIZE: &str = "lfs.maxFileSize";


const SECTION: &str = "twinkle"; // TODO: "sync"?

pub fn key(setting: &str) -> String {
    format!("{SECTION}.{setting}")
}


fn _default_settings()
-> Vec<(String, &'static str)>
{
    // Just here to document all the settings

    vec![
        // Sync
        (key(K_ENABLED), "true"),
        (key(K_ID), "80fa2cca2f73dd2105185daec982df7f20ac372ab5209bece55fdd04dc110c53"),
        (key(K_READONLY), "false"),
        (key(K_POLLING_INTERVAL), "3m"),
        (key(K_LAST_SYNC), "0"),
        (key(K_LAST_CHECK), "0"),

        // Notify
        (key(K_NOTIFY_ENABLED), "true"),
        (key(K_NOTIFY_URL), "wss://notify.sparkleshare.org"),

        // LFS
        (key(K_LFS_ENABLED), "true"),
        (key(K_LFS_SIZE_THRESHOLD), "1m"),
    ]
}


/// Docs: https://git-scm.com/docs/git-config#_variables
pub fn default_git_settings()
-> Vec<(&'static str, String)>
{
    // Important: Keep in sync with the `check` cli command

    vec![
        // Prevent system/global config interference
        ("core.attributesFile", "".into()), // Ignore the system/global attributes files
        ("core.excludesFile", "".into()), // Ignore the system/global gitignore files

        // Cross-platform compatiblity
        ("core.autocrlf", "input".into()), // Text files will keep original line endings when checked out, CRLF chars are normalized to LF when committed
        ("core.fileMode", "false".into()), // Ignore permission changes
        ("core.ignoreCase", "false".into()), // Be case sensitive explicitly to work on macOS
        ("core.precomposeUnicode", "true".into()), // Revert unicode filename decomposition by macOS
        ("core.quotePath", "false".into()), // Output Unicode characters: '"h\303\251"' becomes 'hé'
        ("core.safecrlf", "false".into()),

        ("checkout.workers", recommended_workers().to_string()), // Use more cores than the default "1" when checking out files
        ("push.default", "current".into()), // Push only current branch to matching remote
        ("submodule.recurse", "false".into()), // Ignore submodules

        // Commit signing
        ("commit.gpgSign", "false".into()),
        ("tag.gpgSign", "false".into()),
        ("gpg.format", "ssh".into()),

        // Some memory limiting options
        ("core.bigFileThreshold", "8m".into()),
        ("core.packedGitLimit", "128m".into()),
        ("core.packedGitWindowSize", "128m".into()),
        ("pack.deltaCacheSize", "128m".into()),
        ("pack.packSizeLimit", "128m".into()),
        ("pack.windowMemory", "128m".into()),
    ]
}


fn recommended_workers() -> usize {
    let cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    (cores / 2).clamp(1, 8)
}
