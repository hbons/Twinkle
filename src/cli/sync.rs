//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::env;
use std::path::Path;
use std::time::Duration;

use crate::app::App;

use crate::core::objects::repository::TwinkleRepository;
use crate::core::push;
use crate::core::sync;
use crate::log;


impl App {
    pub async fn cli_command_sync(
        &mut self,
        args: &[String],
    ) -> Result<(), Box<dyn Error>>
    {
        self.cli_require_args(1, args)?;

        let default_path = ".".to_string();
        let path = Path::new(args.get(2).unwrap_or(&default_path));
        let path = self.cli_prepare_path(path)?;

        let interval = args.get(3)
            .and_then(|s| s.strip_prefix("--interval="))
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        let mut repo = TwinkleRepository::new(&path)?;

        if !repo.enabled() {
            return Err("Missing `sync.enabled = true` in git config".into());
        }

        let connection = if repo.push_enabled() {
            let url = repo.push_url()
                .unwrap_or(push::URL_DEFAULT.into());

            // TODO: Allow a disconnected PushConnection,
            //       so we can periodically try to reconnect
            push::connect(&url).await.ok()
        } else {
            None
        };

        if repo.user().is_none() {
            // Maybe a better message?
            log::info(
                "Missing `user.name` and/or `user.email` in git config\n\
                Set one with:\n\
                git config --local user.name \"Your Name\"\n\
                git config --local user.email \"your@email\"",
            );
        }

        let once = env::var("TWINKLE_ONCE").ok().is_some();

        sync::start(
            &mut repo,
            connection,
            interval,
            once,
        ).await
    }
}
