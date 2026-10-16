//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::sync::Arc;

use futures_util::{ SinkExt, StreamExt };
use futures_util::stream::SplitSink;

use tokio;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{ self, Duration };
use tokio_tungstenite;
use tokio_tungstenite::tungstenite::Bytes;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::Message,
};

use crate::log;

use super::objects::channel::TwinkleChannel;
use super::objects::repository::TwinkleRepository;


const PING_INTERVAL_S: u64 = 25;
const RECONNECT_INTERVAL_S: usize = 20;

pub const URL_DEFAULT: &str = "wss://push.sparkleshare.org:443";


const HASH_LEN: usize = 64;
const SPACER: usize = 1;
const MAX_MSG_SIZE: usize = "LISTEN".len() + SPACER + HASH_LEN + SPACER + HASH_LEN;


pub struct PushConnection {
    pub url: Arc<String>,
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    repos: Arc<Mutex<Vec<Arc<TwinkleRepository>>>>,
}


pub async fn connect(url: &str) -> Result<PushConnection, Box<dyn Error>> {
    let config = WebSocketConfig::default()
        .max_frame_size(Some(MAX_MSG_SIZE))
        .max_message_size(Some(MAX_MSG_SIZE));

    let (stream, _response) =
        tokio_tungstenite::connect_async_with_config(
            url,
            Some(config),
            false,
        ).await?;

    let (write, mut read) = stream.split();

    let connection = PushConnection {
        url: Arc::new(url.to_owned()),
        write: Arc::new(Mutex::new(write)),
        repos: Arc::new(Mutex::new(vec![])),
    };

    let url = Arc::clone(&connection.url);
    let write = Arc::clone(&connection.write);
    let repos = Arc::clone(&connection.repos);

    tokio::spawn(async move {
        let mut ping_clock = time::interval(
            Duration::from_secs(PING_INTERVAL_S)
        );

        loop {
            tokio::select! {
                // Read incoming messages
                msg_result = read.next() => {
                    match msg_result {
                        Some(Ok(msg)) => {
                            if let Message::Pong(_) = msg {
                                log::debug(&format!("push | {url} | > Pong!"));
                            }

                            if matches!(msg,
                                Message::Ping(_) |
                                Message::Pong(_) |
                                Message::Close(_)) { // TODO: disconnect on close
                                continue;
                            }


                            log::debug(&format!("push | {url} | > `{}`", msg));

                            for repo in repos.lock().await.iter() {
                                let repo_channel_id = repo.id()
                                    .unwrap()
                                    .parse::<TwinkleChannel>()
                                    .unwrap()
                                    .to_string();

                                if msg.to_string() == repo_channel_id {
                                    log::debug(&format!("push | Matching: `{}`", repo.path.display()));
                                    repo.set_has_remote_changes(true);
                                }
                            }
                        }
                        Some(Err(e)) => {
                            log::error(&format!("push | {url} | Connection dropped: {e}"));
                            break;
                        },
                        None => {
                            log::error(&format!("push | {url} | Connection dropped"));
                            break;
                        }

                    }
                }

                // Ping on interval
                _ = ping_clock.tick() => {
                    let msg = Message::Ping(Bytes::new());
                    log::debug(&format!("push | {url} | Ping?"));

                    if let Err(e) = write.lock().await.send(msg).await {
                        log::error(&format!("push | {url} | Ping fail: {:?}", e));
                        break;
                    }
                }
            }
        }

        dbg!("disconnected"); // TODO: reconnect
    });

    log::debug(&format!("push | Connected: `{}`", connection.url));
    Ok(connection)
}


impl PushConnection {
    pub async fn listen(
        &self,
        channel: &TwinkleChannel,
        repo: Arc<TwinkleRepository>,
    ) -> Result<(), Box<dyn Error>>
    {
        self.repos
            .lock().await
            .push(repo);

        let msg = format!("LISTEN {channel}");

        self.write
            .lock().await
            .send(msg.into()).await?;

        log::debug(&format!("push | Listen: `{channel}`"));
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

        self.write
            .lock().await
            .send(msg.into()).await?;

        log::debug(&format!("push | Notify: `{channel}`"));
        Ok(())
    }
}


impl PushConnection {
    pub async fn ping(&self) -> Result<(), Box<dyn Error>> {
        let msg = Message::Ping(
            Bytes::new(),
        );

        self.write
            .lock().await
            .send(msg).await?; // TODO: write pong on success

        log::debug(&format!("push | Ping! `{}`: Pong!", self.url));
        Ok(())
    }


    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        let mut write = self.write.lock().await;
        write.send(Message::Close(None)).await?;
        write.close().await?;

        log::debug(&format!("push | Disconnected: `{}`", self.url));
        Ok(())
    }
}


pub async fn notify(
    repo: Arc<TwinkleRepository>,
    connection: Arc<Mutex<PushConnection>>,
) -> Result<(), Box<dyn Error>>
{
    tokio::spawn(async move {
        let connection = connection.lock().await;

        if let Some(id) = repo.id() {
            if let Ok(channel) = id.parse::<TwinkleChannel>() {
                _ = connection.notify(&channel, None).await
                    .map_err(|e| log::error("message")); // TODO: show error
            }
        }
    });

    Ok(())
}
