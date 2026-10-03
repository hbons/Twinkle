//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::sync::Arc;

use futures_util::{ SinkExt, StreamExt };
use futures_util::stream::SplitSink;

use tokio::sync::Mutex;
use tokio::net::TcpStream;

use tokio_tungstenite::{
    MaybeTlsStream,
    WebSocketStream,
    connect_async,
    tungstenite::Message,
};

use crate::log;

use super::objects::channel::TwinkleChannel;
use super::objects::repository::TwinkleRepository;


pub struct PushConnection {
    pub url: String,
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    repos: Arc<Mutex<Vec<Arc<TwinkleRepository>>>>,
}


pub async fn connect(
    url: &str,
) -> Result<PushConnection, Box<dyn Error>>
{
    let (stream, _response) = connect_async(url).await?;
    let (write, mut read) = stream.split();

    log::debug(&format!("push | Connected to `{url}`"));

    let connection = PushConnection {
        url: url.to_owned(),
        write: Arc::new(Mutex::new(write)),
        repos: Arc::new(Mutex::new(vec![])),
    };

    let repos_clone = Arc::clone(&connection.repos);

    tokio::spawn(async move {
        while let Some(msg) = read.next().await.and_then(|m| m.ok()) {
            log::debug(&format!("push | Received message `{}`", msg));

            for repo in repos_clone.lock().await.iter() {
                repo.set_has_remote_changes(true);  // TODO: Check channel name
            }
        }
    });

    Ok(connection)
}


impl PushConnection {
    pub async fn listen(
        &mut self,
        channel: &TwinkleChannel,
        repo: Arc<TwinkleRepository>,
    ) -> Result<(), Box<dyn Error>>
    {
        self.repos.lock().await
            .push(repo);

        let msg = format!("LISTEN {channel}");

        let mut write = self.write.lock().await;
        write.send(msg.into()).await?;

        log::debug(&format!("push | Subscribed to channel `{channel}`"));

        Ok(())
    }


    pub async fn notify(
        &self,
        channel: &TwinkleChannel,
        token: Option<&str>,
    ) -> Result<(), Box<dyn Error>>
    {
        let msg = match token {
            Some(t) => format!("NOTIFY {channel} {t}"),
            None    => format!("NOTIFY {channel}"),
        };

        let mut write = self.write.lock().await;
        write.send(msg.into()).await?;

        log::debug(&format!("push | Notified channel `{channel}`"));

        Ok(())
    }
}


impl PushConnection {
    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>>
    {
        let mut write = self.write.lock().await;
        write.send(Message::Close(None)).await?;
        write.close().await?;

        log::debug(&format!("push | Disconnected from `{}`", self.url));

        Ok(())
    }
}
