//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use gio::{
    Cancellable,
    File,
    ListStore,
    Settings,
};

use gio::prelude::*;
use gtk4::{
    gdk::Texture,
    glib::Error,
    FileDialog,
    FileFilter,
    Window,
};

use libadwaita::Avatar;


pub const K_AUTHOR_AVATAR: &str = "author-avatar";


pub fn show_avatar_dialog(
    parent: &Window,
    settings: &Settings,
    avatar: &Avatar,
) {
    let dialog = FileDialog::builder()
        .filters(&filters())
        .modal(true)
        .build();

    let settings = settings.clone();
    let avatar = avatar.clone();

    dialog.open(
        Some(&parent.clone()),
        Some(&Cancellable::new()),
        move |result| {
            if let Err(e) = handle_avatar(result, &settings, &avatar) {
                eprintln!("Could not handle avatar: {e}");
            }
        },
    );
}


fn handle_avatar(
    result: Result<File, Error>,
    settings: &Settings,
    avatar: &Avatar,
) -> Result<(), Box<dyn std::error::Error>>
{
    let file = result?;
    let texture = Texture::from_file(&file)?;

    avatar.set_custom_image(Some(&texture));

    settings.set_string(
        K_AUTHOR_AVATAR,
        &file.uri(),
    )?;

    Ok(())
}


fn filters() -> ListStore {
    let filters = ListStore::new::<FileFilter>();
    filters.append(&filter_image_files());
    filters.append(&filter_all_files());
    filters
}

fn filter_image_files() -> FileFilter {
    let filter = FileFilter::new();
    filter.set_name(Some("Images"));
    filter.add_mime_type("image/*");
    filter
}

fn filter_all_files() -> FileFilter {
    let filter = FileFilter::new();
    filter.set_name(Some("All Files"));
    filter.add_pattern("*");
    filter
}
