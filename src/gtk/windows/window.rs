//   SparkleShare, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;

use gio::{
    Menu,
    SimpleAction,
};

use gtk4::prelude::*;
use gtk4::{
    Align,
    Button,
    MenuButton,
    Orientation,
    PopoverMenu,
    Widget,
    Window,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    Clamp,
    HeaderBar,
    StatusPage,
    ToolbarStyle,
    ToolbarView,
};


pub fn window_new(
    application: &Application,
) -> Result<ApplicationWindow, Box<dyn Error>>
{
    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(640)
        .default_height(540)
        .build();

    let header = HeaderBar::new();
    header.set_widget_name("header_bar");
    header.pack_start(&add_menu_new(application));
    header.pack_end(&main_menu_new(application));


    let toolbar_view = ToolbarView::new();
    toolbar_view.set_widget_name("toolbar_view");
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_top_bar_style(ToolbarStyle::Flat);
    window.add_css_class("flat");

    window.set_content(Some(&toolbar_view));
    // window.add_action(&close_action(&window));

    let app_window = window.clone();
    let window = window.upcast::<Window>();


    window_show_empty_state(&window)?;

    // window.connect_close_request({
    //     // TODO: Remove views and Database from memory on close
    // });

    Ok(app_window)
}


pub fn window_show_empty_state(
    window: &Window,
) -> Result<(), Box<dyn Error>>
{
    let title = "SparkleShare".to_string();

    let button = Button::builder()
        .label("Remote Repository…")
        .css_classes(["pill", "suggested-action"])
        .halign(Align::Fill)
        .build();

    let button2 = Button::builder()
        .label("Local Folder…")
        .css_classes(["pill"])
        .halign(Align::Fill)
        .build();

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    layout.append(&button);
    layout.append(&button2);

    let clamp = Clamp::new();
    clamp.set_maximum_size(200);
    clamp.set_child(Some(&layout));

    let page = StatusPage::builder()
        .icon_name("studio.planetpeanut.SparkleShare")
        .title("Let’s sync!")
        .description("Choose a location to start syncing")
        .child(&clamp)
        .hexpand(false)
        .vexpand(true)
        .build();

    window.set_title(Some(&title));
    window_set_child(window, &page)?;

    Ok(())
}


pub fn main_menu_new(app: &Application) -> MenuButton {
    let button = MenuButton::new();
    button.set_icon_name("open-menu-symbolic");
    button.set_tooltip_text(Some("Main Menu"));

    let button_clone = button.clone();

    let action = SimpleAction::new("open-menu", None);

    action.connect_activate(move |_, _| {
        button_clone.activate();
    });

    app.add_action(&action);
    app.set_accels_for_action("app.open-menu", &["F10"]);

    let menu = Menu::new();
    menu.append(Some("Preferences"), Some("app.preferences"));
    // menu.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
    menu.append(Some("About SparkleShare"), Some("app.about"));

    button.set_popover(
        Some(&PopoverMenu::from_model(Some(&menu)))
    );

    button
}


pub fn add_menu_new(app: &Application) -> MenuButton {
    let button = MenuButton::new();
    button.set_icon_name("list-add-symbolic");
    button.set_tooltip_text(Some("Add"));
    button.set_always_show_arrow(true);

    let button_clone = button.clone();

    let action = SimpleAction::new("open-menu", None);

    action.connect_activate(move |_, _| {
        button_clone.activate();
    });

    app.add_action(&action);

    let menu = Menu::new();
    menu.append(Some("_Remote Repository…"), Some("app.remote-repository"));
    menu.append(Some("_Local Folder…"), Some("app.local-folder"));

    button.set_popover(
        Some(&PopoverMenu::from_model(Some(&menu)))
    );

    button
}



pub fn widget_by_name<T>(
    name: &str, // TODO: swap order
    parent: &impl IsA<Widget>,
) -> Option<T>
where
    T: IsA<Widget>,
{
    let widget = parent.upcast_ref::<Widget>();

    if widget.widget_name() == name {
        return widget.downcast_ref::<T>()
            .cloned();
    }

    let mut child = parent.first_child();

    while let Some(widget) = child {
        if let Some(found) = widget_by_name(name, &widget) {
            return Some(found);
        }

        child = widget.next_sibling();
    }

    None
}


fn window_set_child(
    window: &Window,
    child: &impl IsA<Widget>,
) -> Result<(), Box<dyn Error>>
{
    let toolbar_view =
        widget_by_name::<ToolbarView>(
            "toolbar_view",
            window,
        ).ok_or("Missing ToolbarView named 'toolbar_view'")?;

    toolbar_view.set_content(Some(child));

    Ok(())
}
