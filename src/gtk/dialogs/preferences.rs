//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use gio::Settings;

use gtk4::{
    gdk::Texture,
    Adjustment,
    Align,
    GestureClick,
    Label,
    ListBox,
    Orientation,
    SelectionMode,
    Window,
};

use libadwaita::prelude::*;
use libadwaita::{
    Avatar,
    EntryRow,
    PreferencesDialog,
    PreferencesGroup,
    PreferencesPage,
    SpinRow,
    SwitchRow,
};

use super::avatar::K_AUTHOR_AVATAR;
use super::avatar::show_avatar_dialog;


const K_AUTHOR_NAME: &str = "author-name";
const K_AUTHOR_EMAIL: &str = "author-email";
const K_POLLING_INTERVAL: &str = "polling-interval";
const K_SYNC_IN_BACKGROUND: &str = "sync-in-background";
const K_SYNC_ON_STARTUP: &str = "sync-on-startup";


pub fn show_preferences_dialog(parent: &Window) {
    let settings = Settings::new("studio.planetpeanut.SparkleShare"); // TODO

    let page = PreferencesPage::builder()
        .title("Preferences")
        .icon_name("org.gnome.Settings-system-symbolic")
        .build();

    let group_author = PreferencesGroup::builder()
        .title("Author")
        .build();

    let avatar = Avatar::new(96, Some("Hylke Bons"), false); // TODO: get settings author name
    avatar.set_tooltip_text(Some("Select Avatar")); // TODO: pretty file path if set

    let click = GestureClick::new();

    let path = settings.string(K_AUTHOR_AVATAR);
    let file = gio::File::for_uri(&path.clone());

    match Texture::from_file(&file) {
        Ok(texture) => avatar.set_custom_image(Some(&texture)),
        Err(error) => eprintln!("Could not load avatar image: {error}"),
    };

    let settings2 = settings.clone();
    let avatar2 = avatar.clone();
    let parent2 = parent.clone();
    click.connect_released(move |_, _, _, _| {
        show_avatar_dialog(&parent2, &settings2, &avatar2);
    });

    avatar.add_controller(click);
    avatar.set_cursor_from_name(Some("pointer"));


    let rows = ListBox::new();
    rows.set_selection_mode(SelectionMode::None);
    rows.add_css_class("boxed-list");
    rows.append(&row_name(&settings));
    rows.append(&row_email(&settings));
    rows.set_halign(Align::Fill);

    let content = gtk4::Box::new(Orientation::Horizontal, 24);
    content.append(&avatar);
    content.append(&rows);
    content.set_halign(Align::Fill);
    content.set_valign(Align::Center);
    content.set_hexpand(true);
    content.set_vexpand(false);

    rows.set_valign(Align::Start);
    rows.set_hexpand(true);
    rows.set_vexpand(false);

    group_author.add(&content);


    let group_sync = PreferencesGroup::builder()
        .title("Sync")
        .build();

    let group_sync2 = PreferencesGroup::new();

    group_sync.add(&row_polling_interval(&settings));
    group_sync2.add(&row_sync_in_background(&settings));
    group_sync2.add(&row_sync_on_startup(&settings));

    page.add(&group_author);
    page.add(&group_sync);
    page.add(&group_sync2);


    let preferences = PreferencesDialog::new();
    preferences.add(&page);

    preferences.present(Some(parent));
}


fn row_name(settings: &Settings) -> EntryRow {
    let entry = EntryRow::builder()
        .title("Name")
        .build();

    settings.bind(
        K_AUTHOR_NAME,
        &entry,
        "text"
    ).build();

    entry
}


fn row_email(settings: &Settings) -> EntryRow {
    let entry = EntryRow::builder()
        .title("Email")
        .build();

    settings.bind(
        K_AUTHOR_EMAIL,
        &entry,
        "text"
    ).build();

    entry
}


fn row_polling_interval(settings: &Settings) -> SpinRow {
    let adjustment = Adjustment::builder()
        .lower(10.0)
        .upper(3600.0)
        .step_increment(10.0)
        .build();

    let spin = SpinRow::builder()
        .title("Polling Interval")
        .subtitle("How often to check for remote changes")
        .adjustment(&adjustment)
        .build();

    settings.bind(
        K_POLLING_INTERVAL,
        &spin,
        "value"
    ).build();

    let unit = Label::new(Some("seconds"));
    unit.add_css_class("dimmed");
    spin.add_suffix(&unit);

    spin
}


fn row_sync_in_background(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Sync in Background")
        .subtitle("Keep syncing after closing the app")
        .build();

    settings.bind(
        K_SYNC_IN_BACKGROUND,
        &switch,
        "active"
    ).build();

    switch
}


fn row_sync_on_startup(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Sync on Startup")
        .subtitle("On startup sync in background automatically")
        .build();

    settings.bind(
        K_SYNC_ON_STARTUP,
        &switch,
        "active"
    ).build();

    switch
}
