mod motor;
use motor::MotorController;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
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
    // Initialize the motor controller
    let motors = MotorController::new()?;
    let motors = Arc::new(AsyncMutex::new(motors));

    // Bind a TCP listener for incoming WebSocket connections
    let listener = TcpListener::bind("0.0.0.0:9001").await?;
    println!("WebSocket echo server listening on ws://0.0.0.0:9001");

    // Accept connections in a loop
    while let Ok((stream, _)) = listener.accept().await {
        let motors = motors.clone();
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
                                    // Execute motor command
                                    let mut m = motors.lock().await;
                                    match cmd_msg.payload.as_str() {
                                        "W"    => m.forward(),
                                        "S"    => m.backward(),
                                        "A"    => m.turn_left(),
                                        "D"    => m.turn_right(),
                                        "STOP" => m.stop(),
                                        other   => eprintln!("Unknown command: {}", other),
                                    }
                                }
                            } else {
                                eprintln!("Failed to deserialize as command Msg");
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
