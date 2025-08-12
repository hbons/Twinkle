//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;


pub trait Gui {
    fn gui_init(&self) -> Result<(), Box<dyn Error>>;
    fn gui_run(&self) -> Result<(), Box<dyn Error>>;
    fn gui_run_background(&self) -> Result<(), Box<dyn Error>>;

    fn gui_show_main_window(&self) -> Result<(), Box<dyn Error>>;
    fn gui_show_clone_window(&self) -> Result<(), Box<dyn Error>>;
    fn gui_show_settings_window(&self) -> Result<(), Box<dyn Error>>;

    fn gui_show_notification() -> Result<(), Box<dyn Error>>;

    fn gui_set_folder_icon(&self, path: &Path) -> Result<(), Box<dyn Error>>;
    fn gui_copy_to_clipboard(&self, text: &str) -> Result<(), Box<dyn Error>>;
    fn gui_open_path(&self, path: &Path) -> Result<(), Box<dyn Error>>;
}
