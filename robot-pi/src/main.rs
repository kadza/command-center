use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Bind a TCP listener for incoming WebSocket connections
    let listener = TcpListener::bind("0.0.0.0:9001").await?;
    println!("WebSocket echo server listening on ws://0.0.0.0:9001");

    // Accept connections in a loop
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut ws_stream = accept_async(stream)
                .await
                .expect("Error during the WebSocket handshake");
            println!("New WebSocket connection");

            // Echo incoming text messages back to the client
            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(msg) => {
                        if msg.is_text() {
                            let txt = msg.to_text().unwrap();
                            println!("Received: {}", txt);

                            // Try to parse as JSON, for validation
                            match serde_json::from_str::<Value>(txt) {
                                Ok(v) => println!("Parsed JSON: {:?}", v),
                                Err(_) => println!("Failed to parse JSON"),
                            }

                            // Echo the message back
                            if let Err(e) = ws_stream.send(msg).await {
                                eprintln!("Error sending message: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }

            println!("WebSocket connection closed");
        });
    }

    Ok(())
}
