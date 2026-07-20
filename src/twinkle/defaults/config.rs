//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


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


const SECTION: &str = "twinkle"; // TODO

pub fn key(setting: &str) -> String {
    format!("{SECTION}.{setting}")
}


fn _twinkle_default_settings()
-> Vec<(String, &'static str)>
{
    // Just here to document all the settings

    vec![
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
pub fn twinkle_default_git_settings()
-> Vec<(&'static str, &'static str)>
{
    vec![
        ("core.attributesFile", ""), // Ignore the system and user attributes files
        ("core.autocrlf", "input"), // Text files will keep original line endings when checked out, CRLF chars are normalized to LF when committed
        ("core.excludesFile", ""), // Ignore the system and user gitignore files
        ("core.fileMode", "false"), // Ignore permission changes
        ("core.ignoreCase", "false"), // Be case sensitive explicitly to work on Mac
        ("core.precomposeUnicode", "true"), // Use the same Unicode form on all filesystems
        ("core.quotePath", "false"), // Output Unicode characters: '"h\303\251"' becomes 'hé'
        ("core.safecrlf", "false"),
        ("push.default", "current"), // Push only current branch to matching remote
        ("submodule.recurse", "false"), // Ignore submodules

        // Commit signing
        ("commit.gpgSign", "false"),
        ("tag.gpgSign", "false"),
        ("gpg.format", "ssh"),

        // Some memory limiting options
        ("core.packedGitLimit", "128m"),
        ("core.packedGitWindowSize", "128m"),
        ("pack.deltaCacheSize", "128m"),
        ("pack.packSizeLimit", "128m"),
        ("pack.windowMemory", "128m"),
    ]
}
