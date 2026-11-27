//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::gtk::dialogs::about::show_about_dialog;


pub fn about_action(app: &Application) -> SimpleAction {
    let action = SimpleAction::new("about", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() {
            show_about_dialog(&active_window);
        }
    });

    action
}
