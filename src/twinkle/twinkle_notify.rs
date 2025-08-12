//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{ Config, RecommendedWatcher, RecursiveMode, Result, Watcher };


pub fn twinkle_notify(path: &Path) -> Result<()> {
    let (sender, receiver) = channel();
    let mut watcher = RecommendedWatcher::new(sender, Config::default())?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    loop {
        if let Ok(event) = receiver.recv_timeout(Duration::from_millis(500)) {
            // for path in event.unwrap().paths {
            //     // if !path.components().any(|c| c.as_os_str() == ".git") {
            //     //     continue;
            //     // }

            //     // let e = event;
            //     println!("{:#?}", path);

            // }

            // TODO: Watch the ~/Twinkle folder and parse subdirs
            // Only do this when siblings. may need multiple watchers still

            println!("{:#?}", event);
        }
    }
}
