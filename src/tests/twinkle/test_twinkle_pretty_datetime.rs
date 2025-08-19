//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::twinkle_pretty::twinkle_pretty_datetime;
use chrono::{ TimeZone, Utc, Local };


#[test]
fn test_twinkle_pretty_bool() {
    let seconds_from_epoch = 1_000_000_000;
    let expected = Utc.timestamp_opt(seconds_from_epoch, 0).unwrap().with_timezone(&Local).to_string();
    assert_eq!(twinkle_pretty_datetime(seconds_from_epoch), expected);

    let invalid_seconds = i64::MAX;
    assert_eq!(twinkle_pretty_datetime(invalid_seconds), "Is time even real?");
}
