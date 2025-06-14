use bincode::Decode;
use futures::StreamExt;
use futures::task::Spawn;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

mod class;
use class::Server;

mod packets;
use packets::*;

use crate::config::Config;
mod config;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // console formatting
    tracing_subscriber::fmt()
        .pretty()
        .with_target(true)
        .compact()
        .init();

    let server = Arc::new(Mutex::new(Server::new()));
    let _config = Config::load("../Config.toml");
    let listener = TcpListener::bind("127.0.0.1:8089").await.unwrap();

    tracing::info!("{} Server started. ", server.lock().await.region);

    // spawn the game loop in a separate task to prevent holding up other shit
    let server_clone = Arc::clone(&server);
    tokio::spawn(async move {
        loop {
            {
                let mut server_lock = server_clone.lock().await;
                server_lock.update().await;
            }

            // rust is weird, this escapes the lock above, which frees the server for other stuff
            tokio::time::sleep(std::time::Duration::from_millis(103)).await;
        }
    });

    // "clear" console
    print!("\x1B[2J\x1B[1;1H");

    while let Ok((stream, addr)) = listener.accept().await {
        let server_clone = Arc::clone(&server);

        tokio::spawn(handle_conn(server_clone, stream, addr));
        //span!(Level::WARN, "player connection", id = 1);
        info!(ip = %addr.ip(), "New Player connected");

        let ip = addr.ip();

        if let Err(e) = log_ip_to_file(ip.to_string()) {
            eprintln!("failed to write IP to file: {}", e);
        }
    }
}

async fn handle_conn(server: Arc<Mutex<Server>>, stream: TcpStream, addr: std::net::SocketAddr) {
    let ws = match accept_async(stream).await {
        Ok(wss) => wss,
        Err(error) => {
            eprintln!("error {}", error);
            return;
        }
    };

    let (write, mut read) = ws.split();
    /*
        {
            let mut server_lock = server.lock().await;
            tracing::info!(
                "adding player for {}, id: {}",
                addr,
                server_lock.instance_id
            );
            server_lock.add(write, String::from("bsoias")).await;
        }
    */

    let bincode_settings_standard = bincode::config::standard();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(binary)) => {
                info!("waaa");

                let (packet_type, _): (IncomingPackets, usize) =
                    match bincode::decode_from_slice(&binary, bincode_settings_standard) {
                        Ok(v) => v,
                        Err(err) => {
                            println!("bad err {:?}", err);
                            continue;
                        }
                    };

                info!("waaa");

                match packet_type {
                    IncomingPackets::Spawn(SpawnPacket { name }) => {
                        info!(
                            "adding player for {}",
                            addr
                        );

                        let mut server_lock = server.lock().await;
                        info!(
                            "adding player for {}, id: {}",
                            addr, server_lock.instance_id
                        );
                        server_lock.add(write, name).await;

                        break;
                    }

                    _ => {
                        info!("idk lol");
                    }
                }
            }

            Ok(Message::Text(txt)) => {
                warn!("{}", txt);
            }

            _ => {}
        }
    }
}

fn log_ip_to_file(ip: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ips.log")?;

    writeln!(file, "Client connected: {}", ip)?;
    file.flush()?;
    Ok(())
}
