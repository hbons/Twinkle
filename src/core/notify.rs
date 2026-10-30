//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{
    Config,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use crate::core::objects::repository::TwinkleRepository;
use crate::log;
