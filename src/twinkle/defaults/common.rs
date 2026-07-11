//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::time::Duration;

pub const COMMON_APP_NAME: &str = "Twinkle";
pub const COMMON_MAINTAINER_NAME: &str = "Hylke";
pub const COMMON_MAINTAINER_URL: &str = "https://mastodon.social/@hbons";

pub const COMMON_CONFIG_FILE: &str = ".twinkle/config";

pub const COMMON_FIRST_FILE: &str = "TWINKLE.md";
pub const COMMON_FIRST_COMMIT_MESSAGE: &str = "Set up Twinkle";


// Durations

pub fn twinkle_default_polling_interval() -> Duration {
    Duration::from_secs(90)
}

pub fn twinkle_default_sync_up_delay_max() -> Duration {
    Duration::from_secs(600)
}

pub fn twinkle_default_sync_up_delay_bump() -> Duration {
    Duration::from_secs(15)
}
