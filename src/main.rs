//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_if)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]

pub mod app;
pub mod cli;
pub mod config;
pub mod git;
pub mod gtk;
pub mod gui;
pub mod log;
pub mod ssh;
pub mod twinkle;

mod tests;


use std::env::args;
use std::error::Error;
use std::process::exit;

use crate::app::{ app_deps, app_version };
use crate::app::{ App, app_runs_as_root, app_runs_in_terminal };
use crate::gui::Gui;


fn main() -> Result<(), Box<dyn Error>> {
    log::debug_base(&app_version());
    log::debug_base(&app_deps());

    if app_runs_as_root() {
        log::error_and_exit("Cannot run as root")
    }

    if app_runs_in_terminal() {
        let mut app = App::default();
        let args = args().collect();

        match app.cli_parse_args(&args) {
            Ok(_)  => exit(0),
            Err(e) => log::error_and_exit(&e.to_string())
        };
    }

    let app = App::default();
    app.gui_run()?;

    Ok(())
}
