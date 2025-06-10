use futures::{SinkExt, StreamExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;
use tokio::time::{sleep, Duration};
use serde::Deserialize;

mod class;
use class::Server;

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
    let config = Config::load("../Config.toml");
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

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
    {
        let mut server_lock = server.lock().await;
        tracing::info!("adding player for {}, id: {}", addr, server_lock.instance_id);
        server_lock.add(write).await;
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let data: IncomingPackets = match serde_json::from_str(&text) {
                    Ok(v) => v,
                    Err(_) => {
                        info!("Failed to deserialize incoming message");
                        return;
                    }
                };

                match data {
                    IncomingPackets::Spawn(data) => {
                        info!("woahhh {}", data.name);
                    }

                    _ => {}
                }
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

// packets
#[derive(Deserialize)]
#[serde(tag = "type", content = "data")]
enum IncomingPackets {
    Spawn(SpawnPacket),
    Move,
    Aim,
    Hit,
    Place,
}

#[derive(Deserialize)]
struct SpawnPacket {
    name: String
}