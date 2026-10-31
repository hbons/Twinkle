//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::str::FromStr;

use crate::core::defaults::common::COMMON_CONFIG_FILE;
use crate::core::objects::repository::TwinkleRepository;
use super::config_option::ConfigOption;


impl TwinkleRepository {
    pub fn get_option(&self, name: ConfigOption) -> Option<String> {
        let file = self.path.join(COMMON_CONFIG_FILE);

        // Repositories are allowed to override some options
        if name.allows_override() {
            self.git
                .config_get_with_file(&name.to_string(), &file) // `.twinkle/config`
                .or(self.git.config_get(&name.to_string())) // `.git/config`
        } else {
            self.git.config_get(&name.to_string()) // `.git/config`
        }
    }


    pub fn get_option_as<T>(
        &self,
        name: ConfigOption,
    ) -> Option<T>
    where T: FromStr
    {
        self.get_option(name)?.parse::<T>().ok()
    }
}


impl TwinkleRepository {
    pub fn set_option(
        &self,
        name: ConfigOption,
        value: &str,
    ) -> Result<(), Box<dyn Error>>
    {
        self.git.config_set(&name.to_string(), value)
    }
}


impl ConfigOption {
    pub fn allows_override(&self) -> bool {
        matches!(self,
            Self::SyncId |
            Self::SyncLfsEnabled |
            Self::SyncLfsSizeThreshold |
            Self::SyncPollingInterval |
            Self::SyncPushEnabled |
            Self::SyncPushUrl |
            Self::SyncReadOnly
        )
    }
}
