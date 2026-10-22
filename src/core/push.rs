//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::sync::Arc;

use chrono::Utc;
use futures_util::StreamExt;
use futures_util::stream::SplitStream;

use tokio;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{ self, Duration, Instant };

use tokio_tungstenite;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{ MaybeTlsStream, WebSocketStream };

use crate::log;
use super::objects::push_connection::PushConnection;
use super::objects::repository::TwinkleRepository;


pub const URL_DEFAULT: &str = "wss://push.sparkleshare.org:443";
const MAX_MSG_SIZE: usize = 256;

const PING_INTERVAL: Duration    = Duration::from_secs(25);
const PING_TIMEOUT: Duration     = Duration::from_secs(30);
const TIMEOUT_INTERVAL: Duration = Duration::from_secs(5);


pub async fn connect(
    url: &str,
) -> Result<Arc<Mutex<PushConnection>>, Box<dyn Error + Send + Sync>>
{
    let url = url.to_owned();
    let (connection, mut read) = create_connection(&url).await?;

    let connection  = Arc::new(Mutex::new(connection));
    let conn_c1 = Arc::clone(&connection);
    let conn_c2 = Arc::clone(&connection);

    tokio::spawn(async move {
        loop {
            handle_connection(&conn_c2, &mut read).await;

            let delay = reconnect_delay();
            log::debug(&format!("push | {url} | Reconnecting in {}s", delay.as_secs()));
            time::sleep(delay).await;

            if let Ok((mut c, r)) = create_connection(&url).await {
                let repos = conn_c2.lock().await.repos.clone();
                c.repos = repos.clone();

                *conn_c2.lock().await = c;
                read = r;

                for repo in repos {
                    if let Some(channel) = repo.channel() {
                        _ = conn_c2.lock().await.listen(
                            &channel,
                            Arc::clone(&repo)
                        ).await;
                    }
                }
            }
        }
    });

    Ok(conn_c1)
}


/// Pseudo-random delay up to 8192ms
fn reconnect_delay() -> Duration {
    let hex = 0x1FFF; // 8192 values
    let nano_seconds = Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or(8192);

    Duration::from_millis((nano_seconds & hex) as u64)
}


async fn create_connection(
    url: &str,
) -> Result<(PushConnection, SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>), Box<dyn Error + Send + Sync>>
{
    let config = WebSocketConfig::default()
        .max_frame_size(Some(MAX_MSG_SIZE))
        .max_message_size(Some(MAX_MSG_SIZE));

    let (stream, _) = time::timeout(Duration::from_secs(4),
        tokio_tungstenite::connect_async_with_config(url, Some(config), false)).await
        .map_err(|_| "Timeout trying to connect: 4s")??;

    log::debug(&format!("push | Connected: `{}`", url));
    let (write, read) = stream.split();

    Ok((
        PushConnection::new(url, write),
        read,
    ))
}


async fn handle_connection(
    connection: &Arc<Mutex<PushConnection>>,
    read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    let mut timeout_clock = time::interval_at(
        Instant::now() + TIMEOUT_INTERVAL,
        TIMEOUT_INTERVAL,
    );

    let mut ping_clock = time::interval_at(
        Instant::now() + PING_INTERVAL,
        PING_INTERVAL,
    );

    loop {
        tokio::select! {
            // Incoming messages
            result = read.next() => {
                let url = connection.lock().await.url.clone();

                match result {
                    Some(Ok(msg)) =>
                        match handle_message(msg, Arc::clone(connection)).await {
                            Ok(_)  => continue,
                            Err(_) => break,
                        },
                    Some(Err(e)) => {
                        log::error(&format!("push | {url} | Connection error: {e}"));
                        break;
                    },
                    None => {
                        log::error(&format!("push | {url} | Connection error"));
                        break;
                    }
                }
            }

            // Check timeout
            _ = timeout_clock.tick() => {
                let url = connection.lock().await.url.clone();

                let inactive_time = {
                    let last_pong = connection.lock().await.last_pong;

                    Utc::now().timestamp()
                        .saturating_sub(last_pong) as u64
                };

                if inactive_time >= PING_TIMEOUT.as_secs() {
                    log::debug(&format!("push | {url} | Timed out: {}s", PING_TIMEOUT.as_secs()));
                    break;
                }
            }

            // Interval ping
            _ = ping_clock.tick() => {
                let url = connection.lock().await.url.clone();

                if let Err(e) = connection.lock().await.ping().await {
                    log::error(&format!("push | {url} | Ping fail: {:?}", e));
                    break;
                }
            }
        }
    }
}


async fn handle_message(
    msg: Message,
    connection: Arc<Mutex<PushConnection>>,
) -> Result<(), Box<dyn Error>>
{
    let url = connection.lock().await.url.clone();

    match msg {
        Message::Pong(_) => {
            connection.lock().await.last_pong = Utc::now().timestamp();
            log::debug(&format!("push | {url} | > Pong!"));
        },
        Message::Close(_) => {
            log::debug(&format!("push | {url} | Close"));
            return Err("Close".into());
        },
        Message::Text(text) => {
            log::debug(&format!("push | {url} | > `{}`", text));

            for repo in connection.lock().await.repos.iter() {
                let Some(channel) = repo.channel() else {
                    continue;
                };

                if text == channel.to_string() {
                    log::debug(&format!("push | Matches: `{}`", repo.path.display()));
                    repo.set_has_remote_changes(true);
                }
            }
        },
        _ => (),
    }

    Ok(())
}


pub async fn send_notify(
    repo: Arc<TwinkleRepository>,
    connection: Arc<Mutex<PushConnection>>,
) -> Result<(), Box<dyn Error>>
{
    tokio::spawn(async move {
        let Some(channel) = repo.channel() else {
            log::error("push | Could not get channel for repo");
            return;
        };

        _ = connection
            .lock().await
            .notify(&channel, None).await
            .map_err(|e| log::error(&format!("push | Could not notify: {e:?}")));
    });

    Ok(())
}
