//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::fmt;


pub enum ConfigOption {
    CommitGpgSign,
    TagGpgSign,
    CoreIgnoreCase,
    CoreSshCommand,
    UserName,
    UserEmail,
    UserSigningKey,
    RemoteOriginUrl,

    // Sync
    SyncEnabled,
    SyncId,
    SyncLastCheck,
    SyncLastSync,
    SyncLfsEnabled,
    SyncLfsSizeThreshold,
    SyncPollingInterval,
    SyncPushEnabled,
    SyncPushUrl,
    SyncReadOnly,
}


impl fmt::Display for ConfigOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CommitGpgSign   => "commit.gpgSign",
            Self::TagGpgSign      => "tag.gpgSign",
            Self::CoreIgnoreCase  => "core.ignoreCase",
            Self::CoreSshCommand  => "core.sshCommand",
            Self::UserName        => "user.name",
            Self::UserEmail       => "user.email",
            Self::UserSigningKey  => "user.signingKey",
            Self::RemoteOriginUrl => "remote.origin.url",

            // Sync
            Self::SyncEnabled          => "sync.enabled",
            Self::SyncId               => "sync.id",
            Self::SyncLastCheck        => "sync.lastCheck",
            Self::SyncLastSync         => "sync.lastSync",
            Self::SyncLfsEnabled       => "sync.lfs.enabled",
            Self::SyncLfsSizeThreshold => "sync.lfs.sizeThreshold",
            Self::SyncPollingInterval  => "sync.pollingInterval",
            Self::SyncPushEnabled      => "sync.push.enabled",
            Self::SyncPushUrl          => "sync.push.url",
            Self::SyncReadOnly         => "sync.readOnly",
        };

        f.write_str(s)
    }
}
