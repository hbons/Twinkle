//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::time::Duration;

use crate::twinkle::twinkle_default::twinkle_default_sync_up_delay_bump;
use crate::twinkle::twinkle_default::twinkle_default_sync_up_delay_max;
use crate::twinkle::twinkle_watch::twinkle_sync_up_delay;


#[test]
fn test_twinkle_sync_up_delay() {
    let attempts = 0;
    let delay = twinkle_sync_up_delay(attempts);
    assert_eq!(delay, Duration::from_secs(0));

    let attempts = 1;
    let delay = twinkle_sync_up_delay(attempts);
    assert_eq!(delay, Duration::from_secs(0));

    let attempts = 5;
    let delay = twinkle_sync_up_delay(attempts);
    assert_eq!(delay, Duration::from_secs(twinkle_default_sync_up_delay_bump().as_secs() * (attempts - 1)));

    let attempts = twinkle_default_sync_up_delay_max().as_secs() / twinkle_default_sync_up_delay_bump().as_secs() + 1;
    let delay = twinkle_sync_up_delay(attempts);
    assert_eq!(delay, twinkle_default_sync_up_delay_max());

    let attempts = twinkle_default_sync_up_delay_max().as_secs() / twinkle_default_sync_up_delay_bump().as_secs() + 10;
    let delay = twinkle_sync_up_delay(attempts);
    assert_eq!(delay, twinkle_default_sync_up_delay_max());
}
