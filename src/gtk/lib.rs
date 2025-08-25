//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


// use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

// use gio::prelude::*;
// use gio;
// use gtk::prelude::*;
// use gtk::{ Builder, Window, WindowType };

// use zbus::blocking::Connection;
// use zbus::blocking::Proxy;
// use zvariant::Value;

use crate::app::App;
use crate::gui::Gui;


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4/
    //       https://docs.rs/zbus/latest/zbus/

    fn gui_init(&self) -> Result<(), Box<dyn Error>> {
        // gio::init()?;
        // gtk::init()?

        Ok(())
    }


    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        // gtk::main();
        Ok(())
    }


    fn gui_run_background(&self) -> Result<(), Box<dyn Error>> {
        // let connection = Connection::session()?;

        // let proxy = Proxy::new(
        //     &connection,
        //     "org.freedesktop.portal.Desktop", // D-Bus service
        //     "/org/freedesktop/portal/desktop", // Object path
        //     "org.freedesktop.portal.Background", // Interface
        // )?;

        // let reason = format!("{} can run in the background to watch for local and remote changes.", self.name);
        // let options: HashMap<&str, Value> = HashMap::new();

        // let response: String = proxy.call("RequestBackground", &(
        //     self.id,
        //     reason,
        //     options))?;

        // println!("Background request response: {}", response);
        Ok(())
    }


    fn gui_set_autostart(&self, _value: bool) -> Result<(), Box<dyn Error>> {
        // set via dbus
        Ok(())
    }


    fn gui_show_main_window(&self) -> Result<(), Box<dyn Error>> {
        // let builder = Builder::new_from_file("resources/main.ui");
        // let window: Window = builder.get_object("main_window")?;

        // window.connect_delete_event(|_, _| {
        //      gtk::main_quit();
        //      Inhibit(false)
        // });

        // window.show_all();

        Ok(())
    }


    fn gui_show_clone_window(&self) -> Result<(), Box<dyn Error>> {
        // let builder = Builder::new_from_file("resources/clone.ui");
        // let window: Window = builder.get_object("clone_window")?;

        // window.connect_url_entry_changed
        // window.connect.trust_clicked
        // window.connect.copy_clicked
        // window.connect_cancel_clicked
        // window.connect_clone_clicked

        // window.show_all();

        Ok(())
    }


    fn gui_show_settings_window(&self) -> Result<(), Box<dyn Error>> {
        // let builder = Builder::new_from_file("resources/settings.ui");
        // let window: Window = builder.get_object("settings_window")?;
        // window.show_all();

        Ok(())
    }


    fn gui_show_notification() -> Result<(), Box<dyn Error>> {
        // let connection = Connection::session().await?;

        // // Create a proxy to org.freedesktop.Notifications
        // let proxy = zbus::Proxy::new(
        //     &connection,
        //     "org.freedesktop.Notifications",        // destination (notification daemon)
        //     "/org/freedesktop/Notifications",       // object path
        //     "org.freedesktop.Notifications"         // interface name
        // ).await?;

        // // Call the Notify method
        // let id: u32 = proxy
        //     .call(
        //         "Notify",
        //         &(
        //             "my-app",          // app_name
        //             0u32,              // replaces_id
        //             "dialog-information", // icon
        //             "Hello from zbus!",   // summary
        //             "This is a raw D-Bus notification", // body
        //             Vec::<&str>::new(), // actions
        //             std::collections::HashMap::<&str, Value>::new(), // hints
        //             5000i32            // expire_timeout (ms)
        //         ),
        //     )
        //     .await?;

        // println!("Notification shown with ID: {id}");

        Ok(())
    }


    fn gui_set_folder_icon(&self, _path: &Path) -> Result<(), Box<dyn Error>> {
        // Docs: https://docs.gtk.org/gio/

        let _attribute = "metadata::custom-icon-name";
        let _icon_name = &self.icon;

        // let folder = gio::File::for_path(path);
        // let info = folder.query_info(attribute, gio::FileQueryInfoFlags::NONE, None)?;
        // info.set_attribute_string(attribute, icon_name);
        // folder.set_attributes_from_info(&info, gio::FileSetAttributesFlags::NONE)?;

        Ok(())
    }


    fn gui_copy_to_clipboard(&self, _text: &str) -> Result<(), Box<dyn Error>> {
        // let clipboard = Clipboard::get(&SelectionTarget::clipboard());
        // clipboard.set_text(text);

        Ok(())
    }


    fn gui_open_path(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let _uri = format!("file://{}", path.display());
        // gio::AppInfo::launch_default_for_uri(&uri)?;

        Ok(())
    }
}


#[allow(dead_code)]
struct StatusWidget {
    needs_redraw: bool,
    enabled: bool,

    icon: String,
    title: String,
    details: String,

    status: String,
    status_details: String,
    progress: Option<u32>,

    menu_enabled: bool,
}


#[allow(dead_code)]
struct AuthorWidget {
    needs_redraw: bool,
    enabled: bool,

    name: String,
    email: String,
}
