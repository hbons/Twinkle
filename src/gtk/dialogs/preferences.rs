//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use gio::{
    Settings,
    SettingsSchemaSource
};

use gtk4::{
    glib::Variant,
    StringList,
    Window
};

use libadwaita::prelude::*;
use libadwaita::{
    ComboRow,
    PreferencesDialog,
    PreferencesGroup,
    PreferencesPage,
    SpinRow,
    SwitchRow,
};


pub fn show_preferences_dialog(parent: &Window) {
    let settings = Settings::new("studio.planetpeanut.SparkleShare"); // TODO


    let page = PreferencesPage::builder()
        .title("Preferences")
        .icon_name("org.gnome.Settings-system-symbolic")
        .build();


    let group_author = PreferencesGroup::builder()
        .title("Author")
        .build();

    group_author.add(&row_numbers(&settings));
    group_author.add(&row_order(&settings));


    let group_sync = PreferencesGroup::builder()
        .title("Sync")
        .build();

    group_sync.add(&row_polling_interval(&settings));
    group_sync.add(&row_sync_in_background(&settings));
    group_sync.add(&row_sync_on_startup(&settings));

    page.add(&group_author);
    page.add(&group_sync);


    let preferences = PreferencesDialog::new();
    preferences.add(&page);

    preferences.present(Some(parent));
}


fn row_numbers(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Row Numbers")
        .build();

    settings.bind(
        "row-numbers",
        &switch,
        "active"
    ).build();

    switch
}

fn row_order(settings: &Settings) -> ComboRow {
    combo_row_with_binding(
        settings,
        "row-order",
        "Row Order",
        None,
        &["Newest First",
        "Oldest First"],
    )
}

fn row_separator(settings: &Settings) -> ComboRow {
    combo_row_with_binding(
        settings,
        "column-separator",
        "Column Separator",
        Some("Used when copying rows to the clipboard"),
        &["Tabs", "Spaces", "Commas", "Markdown"],
    )
}


fn row_polling_interval(settings: &Settings) -> SpinRow {
    let adjustment = gtk4::Adjustment::builder()
        .lower(0.0)
        .upper(100.0)
        .step_increment(1.0)
        .value(3.0)
        .build();

    let switch = SpinRow::builder()
        .title("Polling Interval")
        .adjustment(&adjustment)
        .build();

    settings.bind(
        "monospace-font",
        &switch,
        "active"
    ).build();

    switch
}


fn row_sync_in_background(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Sync in Background")
        .subtitle("Continue sync when the window is closed")
        .build();

    settings.bind(
        "monospace-font",
        &switch,
        "active"
    ).build();

    switch
}


fn row_sync_on_startup(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Sync on Startup")
        .subtitle("Start sync in the background on startup")
        .build();

    settings.bind(
        "monospace-font",
        &switch,
        "active"
    ).build();

    switch
}


fn combo_row_with_binding(
    settings: &Settings,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    choices: &[&str],
) -> ComboRow
{
    let value = settings.string(key).to_string();
    let index = value_to_index(settings, key, &value);

    let combo = ComboRow::builder()
        .title(title)
        .subtitle(subtitle.unwrap_or(""))
        .model(&StringList::new(choices))
        .build();

    if let Some(i) = index {
        combo.set_selected(i);
    }

    let key_handle = key.to_string();
    let settings_handle = settings.clone();

    combo.connect_notify_local(Some("selected"), move |row, _| {
        let index = row.selected();

        if let Some(value) = index_to_value(&settings_handle, &key_handle, index) {
            _ = settings_handle.set_string(&key_handle, &value);
        }
    });

    combo
}


fn choices_for_key(settings: &Settings, key: &str) -> Option<Vec<String>> {
    let source = SettingsSchemaSource::default()?;
    let schema = source.lookup(settings.schema_id()?.as_ref(), true)?;
    let schema_key = schema.key(key);

    let range = schema_key.range();
    let (_kind, value): (String, Variant) = range.get()?;

    value.get()
}

fn index_to_value(settings: &Settings, key: &str, index: u32) -> Option<String> {
    let choices = choices_for_key(settings, key)?;
    choices.get(index as usize).cloned()
}

fn value_to_index(settings: &Settings, key: &str, value: &str) -> Option<u32> {
    let choices = choices_for_key(settings, key)?;
    let index = choices.iter().position(|v| v == value)?;

    Some(index as u32)
}
