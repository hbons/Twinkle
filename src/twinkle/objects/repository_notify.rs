//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use sha2::{ Sha256, Digest };
use crate::twinkle::objects::repository::TwinkleRepository;


pub type TwinkleChannelName = String;

// Convenience
impl TwinkleRepository {
    /// Channel name for notifications
    /// SHA-256 of the repository id
    pub fn as_notify_channel_name(&self) -> Option<TwinkleChannelName> {
        self.id().map(|s|
            Sha256::digest(s)
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect()
        )
    }
}
