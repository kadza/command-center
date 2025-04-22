use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use serde::Deserialize;
use anyhow::Result;

// Define the JSON message structure for commands
#[derive(Deserialize)]
struct Msg {
    #[serde(rename = "type")]
    msg_type: String,
    payload: String,
}

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
                            // Try to parse as command message
                            if let Ok(cmd_msg) = serde_json::from_str::<Msg>(txt) {
                                if cmd_msg.msg_type == "cmd" {
                                    match cmd_msg.payload.as_str() {
                                        "W"    => println!("▶ Motor stub: FORWARD"),
                                        "S"    => println!("▶ Motor stub: BACKWARD"),
                                        "A"    => println!("▶ Motor stub: TURN LEFT"),
                                        "D"    => println!("▶ Motor stub: TURN RIGHT"),
                                        "STOP" => println!("▶ Motor stub: STOP ALL MOTORS"),
                                        other   => println!("▶ Motor stub: UNKNOWN `{}`", other),
                                    }
                                }
                            } else {
                                println!("Failed to deserialize as command Msg");
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
