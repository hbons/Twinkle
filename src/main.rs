//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_if)]

#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
// #![deny(clippy::arithmetic_side_effects)]


pub mod core;
pub mod cli;
pub mod git;
pub mod gtk;
pub mod gui;
pub mod log;
pub mod runtime;
pub mod ssh;

mod tests;


use std::env::args;
use std::error::Error;
use std::process::exit;

use crate::runtime::Runtime;
use crate::gui::Gui;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::debug_base(&runtime::version());
    log::debug_base(&runtime::deps());

    if runtime::runs_as_root() {
        log::error_and_exit("Cannot run as root")
    }

    if let Some(arg) = args().nth(1) {
        if arg == "gui" {
            let app = Runtime::default();
            app.gui_run()?;
            exit(0);
        }
    }

    let mut app = Runtime::default();
    let args: Vec<String> = args().collect();

    match app.cli_parse_args(&args).await {
        Ok(_)  => exit(0),
        Err(e) => log::error_and_exit(&e.to_string())
    };
}
