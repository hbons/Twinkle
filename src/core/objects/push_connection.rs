//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::sync::Arc;

use chrono::Utc;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;

use tokio;
use tokio::net::TcpStream;

use tokio_tungstenite;
use tokio_tungstenite::{ MaybeTlsStream, WebSocketStream };
use tokio_tungstenite::tungstenite::{ Bytes, Message };

use crate::log;
use super::channel::TwinkleChannel;
use super::repository::TwinkleRepository;


#[derive(Debug)]
pub struct PushConnection {
    pub url: String,

    /// Timestamp of last response to our pings
    pub last_pong: i64,

    /// Used to send messages to the service
    pub write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,

    /// List of repositories that are listening over this connection
    pub repos: Vec<Arc<TwinkleRepository>>,
}


impl PushConnection {
    pub fn new(
        url: &str,
        write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
   ) -> Self
   {
       Self {
           url: url.to_string(),
           last_pong: Utc::now().timestamp(),
           write,
           repos: vec![],
       }
   }
}


impl PushConnection {
    pub async fn listen(
        &mut self,
        channel: &TwinkleChannel,
        repo: Arc<TwinkleRepository>,
    ) -> Result<(), Box<dyn Error>>
    {
        let msg = format!("LISTEN {channel}");
        self.write.send(msg.into()).await?;

        log::debug(&format!("push | Listen: `{channel}`"));

        if self.repos
            .iter()
            .find(|r| r.id() == repo.id())
            .is_none()
        {
            self.repos.push(repo);
        }

        Ok(())
    }


    pub async fn notify(
        &mut self,
        channel: &TwinkleChannel,
        token: Option<&str>,
    ) -> Result<(), Box<dyn Error>>
    {
        let msg = match token {
            Some(t) => format!("NOTIFY {channel} {t}"),
            None    => format!("NOTIFY {channel}"),
        };

        self.write.send(msg.into()).await?;
        log::debug(&format!("push | Notify: `{channel}`"));

        Ok(())
    }
}


impl PushConnection {
    pub async fn ping(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = Message::Ping(Bytes::new());
        self.write.send(msg).await?;

        log::debug(&format!("push | {} | Ping!", self.url));

        Ok(())
    }
}


impl PushConnection {
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        self.write.send(Message::Close(None)).await?;
        self.write.close().await?;

        log::debug(&format!("push | Disconnected: `{}`", self.url));

        Ok(())
    }
}
