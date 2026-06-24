//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


/// Docs: https://git-scm.com/docs/git-config#_variables
pub fn twinkle_default_settings()
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
