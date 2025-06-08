use futures::{SinkExt, StreamExt};
use std::fs::OpenOptions;
use std::io::Write;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{Level, debug, error, info, span, warn};
use tracing_subscriber::prelude::*;
use tracing_tree::HierarchicalLayer;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");

    //tracing_subscriber::registry().with(HierarchicalLayer::new(2).with_indent_lines(true).with_targets(true)).init();
    tracing_subscriber::fmt()
        .pretty()
        .with_target(true)
        .compact()
        .init();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("Server started!");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_conn(stream));
        //span!(Level::WARN, "player connection", id = 1);
        info!(ip = %addr.ip(), "New Player connected");
        let ip = addr.ip();

        if let Err(e) = log_ip_to_file(ip.to_string()) {
            eprintln!("failed to write IP to file: {}", e);
        }
    }
}

async fn handle_conn(stream: TcpStream) {
    let ws = match accept_async(stream).await {
        Ok(wss) => wss,
        Err(error) => {
            eprintln!("error {}", error);
            return;
        }
    };

    let (mut write, mut read) = ws.split();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("woahh {}", text);
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
