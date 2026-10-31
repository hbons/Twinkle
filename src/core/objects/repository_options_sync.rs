//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::time::Duration;

use crate::core::lfs::LFS_THRESHOLD;
use crate::core::defaults::common::default_polling_interval;

use super::repository::TwinkleRepository;
use super::config_option::ConfigOption;


// SyncId
impl TwinkleRepository {
    pub fn id(&self) -> Option<String> { // TODO: Option<Sha256>
        self.get_option_as(ConfigOption::SyncId)
    }

    pub fn set_id(&self, value: &str) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncId, value)
    }
}


// SyncEnabled
impl TwinkleRepository {
    pub fn enabled(&self) -> bool {
        self.get_option_as::<bool>(ConfigOption::SyncEnabled)
            .unwrap_or_default()
    }

    pub fn set_enabled(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncEnabled, &value.to_string())
    }
}


// SyncReadOnly
impl TwinkleRepository {
    pub fn read_only(&self) -> bool {
        self.get_option_as::<bool>(ConfigOption::SyncReadOnly)
            .unwrap_or_default()
    }

    pub fn set_read_only(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncReadOnly, &value.to_string())
    }
}


// SyncLastCheck and SyncLastSync
impl TwinkleRepository {
    pub fn last_check(&self) -> Option<i64> {
        self.get_option_as::<i64>(ConfigOption::SyncLastCheck)
    }

    pub fn set_last_check(&self, value: i64) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncLastCheck, &value.to_string())
    }


    pub fn last_sync(&self) -> Option<i64> {
        self.get_option_as::<i64>(ConfigOption::SyncLastSync)
    }

    pub fn set_last_sync(&self, value: i64) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncLastSync, &value.to_string())
    }
}


// SyncPushEnabled and SyncPushUrl
impl TwinkleRepository {
    pub fn push_enabled(&self) -> bool {
        self.get_option_as::<bool>(ConfigOption::SyncPushEnabled)
            .unwrap_or_default()
    }

    pub fn set_push_enabled(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncPushEnabled, &value.to_string())
    }


    pub fn push_url(&self) -> Option<String> {
        self.get_option(ConfigOption::SyncPushUrl)
    }

    pub fn set_push_url(&self, value: &str) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncPushUrl, value)
    }
}


// SyncLfsEnabled and SyncLfsSizeThreshold
impl TwinkleRepository {
    pub fn lfs_enabled(&self) -> bool {
        self.get_option_as::<bool>(ConfigOption::SyncLfsEnabled).unwrap_or_default()
    }

    pub fn set_lfs_enabled(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncLfsEnabled, &value.to_string())
    }


    pub fn lfs_size_threshold(&self) -> u64 {
        let Some(s) = self.get_option(ConfigOption::SyncLfsSizeThreshold) else { return LFS_THRESHOLD };
        parse_lfs_size(&s)
    }

    pub fn set_lfs_size_threshold(&self, value: u64) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncLfsSizeThreshold, &value.to_string())
    }
}

pub fn parse_lfs_size(s: &str) -> u64 {
    let (number, multiplier) = match s.as_bytes().last() {
        Some(b'k' | b'K') => (&s[..s.len() - 1], 1024),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 1024 * 1024),
        Some(b'g' | b'G') => (&s[..s.len() - 1], 1024 * 1024 * 1024),
        _ => (s, 1),
    };

    number.parse::<u64>().unwrap_or(0) * multiplier
}


// SyncPollingInterval
impl TwinkleRepository {
    pub fn polling_interval(&self) -> Duration {
        let Some(s) = self.get_option(ConfigOption::SyncPollingInterval) else {
            return default_polling_interval()
        };

        parse_polling_interval(&s)
    }

    pub fn set_polling_interval(&self, value: Duration) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::SyncPollingInterval, &value.as_secs().to_string())
    }
}

pub fn parse_polling_interval(s: &str) -> Duration {
    let (number, multiplier) = match s.as_bytes().last() {
        Some(b's' | b'S') => (&s[..s.len() - 1], 1),
        Some(b'm' | b'M') => (&s[..s.len() - 1], 60),
        Some(b'h' | b'H') => (&s[..s.len() - 1], 60 * 60),
        _ => (s, 1),
    };

    Duration::from_secs(
        number.parse::<u64>().unwrap_or(0) * multiplier
    )
}
