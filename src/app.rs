//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::{ Path, PathBuf };
use std::process::Command;

use crate::git::objects::environment::GitEnvironment;
use crate::log;
use crate::ssh::version::ssh_version;
use crate::twinkle::twinkle_config::TwinkleConfig;


#[derive(Debug)]
pub struct App {
    pub id:      String,
    pub name:    String,
    pub command: String,
    pub icon:    String,
    pub version: String,

    // Docs: https://docs.flatpak.org/en/latest/sandbox-permissions.html
    pub is_flatpak: bool,

    // Installation
    pub prefix:       PathBuf,
    pub desktop_file: PathBuf,
    pub appdata_file: PathBuf,

    // Runtime
    pub repos_home:      PathBuf,
    pub app_config_home: PathBuf,
    pub app_keys_dir:    PathBuf,
    pub repos_file:      PathBuf,
    pub app_data_home:   PathBuf,
    pub app_cache_home:  PathBuf,

    pub config: TwinkleConfig,
}


#[allow(clippy::expect_used)]
impl Default for App {
    fn default() -> Self {
        let app_id = "studio.planetpeanut.Twinkle";

        let app_name     = env!("CARGO_PKG_NAME");
        let command_name = env!("CARGO_BIN_NAME");
        let version      = env!("CARGO_PKG_VERSION");

        let prefix = Path::new("/app");


        // HOME
        let home_dir = env::var("HOME").unwrap_or_else(|_| {
            log::error_and_exit("Could not read HOME environment variable")
        });

        let home_dir = Path::new(&home_dir);

        // XDG
        let mut xdg_documents_dir = home_dir.to_path_buf();
        let mut xdg_config_home   = home_dir.join(".config");      // ~/.var/app/<APP_ID>/config
        let mut xdg_data_home     = home_dir.join(".local/share"); // ~/.var/app/<APP_ID>/data
        let mut xdg_cache_home    = home_dir.join(".cache");       // ~/.var/app/<APP_ID>/cache

        // Flatpak
        if let Ok(var) = env::var("XDG_DOCUMENTS_DIR") { xdg_documents_dir = Path::new(&var).into(); }
        if let Ok(var) = env::var("XDG_CONFIG_HOME") { xdg_config_home = Path::new(&var).into(); }
        if let Ok(var) = env::var("XDG_DATA_HOME") { xdg_data_home = Path::new(&var).into(); }
        if let Ok(var) = env::var("XDG_CACHE_HOME") { xdg_cache_home = Path::new(&var).into(); }

        // Config
        let config_path = xdg_config_home.join(format!("{command_name}/repos.json"));
        let mut config = TwinkleConfig::new(&config_path);
        _ = config.load();

        App {
            id:      app_id.into(),
            name:    app_name.into(),
            command: command_name.into(),
            icon:    app_id.into(),
            version: version.into(),

            is_flatpak: app_is_flatpak(),

            // Installation
            prefix:       prefix.into(),
            desktop_file: prefix.join(format!("share/applications/{command_name}.desktop")),
            appdata_file: prefix.join(format!("share/appdata/{command_name}.appdata.xml")),

            // Runtime
            repos_home:      xdg_documents_dir.join(app_name),
            app_config_home: xdg_config_home.join(format!("{command_name}")),
            app_keys_dir:    xdg_config_home.join(format!("{command_name}/keys")),
            repos_file:      xdg_config_home.join(format!("{command_name}/repos.json")),
            app_data_home:   xdg_data_home.join(format!("{command_name}")),
            app_cache_home:  xdg_cache_home.join(format!("{command_name}")),

            config,
        }
    }
}


pub fn app_runs_as_root() -> bool {
    match Command::new("id").arg("-u").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "0",
        Err(_) => true, // Assume root if we can't check
    }
}


pub fn app_runs_in_terminal() -> bool {
    env::var("TERM").is_ok()
}


pub fn app_version() -> String {
    let mut app_str = format!("{} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));

    if app_is_flatpak() {
        app_str.push_str(" (Flatpak)");
    }

    app_str
}


pub fn app_deps() -> String {
    let git = GitEnvironment::default();

    format!("{}\n{}\n{}",
        ssh_version(),
        git.version(),
        git.lfs_version())
}


pub fn app_is_flatpak() -> bool {
    // Docs: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak-run
    env::var("FLATPAK_ID").is_ok()
}
