//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

use crate::app::App;

use crate::core::objects::repository::TwinkleRepository;
use crate::core::push;
use crate::core::sync;


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

        let push_url = repo.push_url()
            .unwrap_or(push::DEFAULT_SERVER.into());

        let connection = Arc::new(Mutex::new(
            push::connect(&push_url).await?
        ));

        if !repo.enabled() {
            return Err("Sync not enabled on repository".into());
        }

        // TODO: Stop if no user set or let git commit fail?

        let once = env::var("TWINKLE_ONCE").ok();
        sync::start(&mut repo, Some(connection), interval, once.is_some())
    }
}
