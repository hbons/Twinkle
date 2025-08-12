//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


#[derive(Debug)]
pub enum GitError {
    NoConnection,
    HostKeyChanged,
    PermissionDenied,
    UnreadableFiles,
    NotFound,
    DiskSpaceExceeded,
    Other(String),
}
