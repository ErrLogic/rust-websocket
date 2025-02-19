use std::sync::Arc;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use serde_json::{json, Value};
use tokio::sync::{Mutex, broadcast};
use warp::ws::Message;
use warp::ws::WebSocket;

pub async fn subscribe_channel(tx: Arc<Mutex<SplitSink<WebSocket, Message>>>, mut receiver: broadcast::Receiver<String>) {
    while let Ok(msg) = receiver.recv().await {
        let mut tx = tx.lock().await;

        let formatted_msg = match serde_json::from_str::<Value>(&msg) {
            Ok(data) => {
                let sender = data.get("sender")
                    .and_then(Value::as_str)
                    .unwrap_or("Unknown")
                    .to_string();

                let message = data.get("message")
                    .map(|m| m.to_string())
                    .unwrap_or("\"No message\"".to_string());

                json!({
                    "sender": sender,
                    "message": message
                })
                    .to_string()
            }
            Err(_) => {
                json!({
                    "sender": "Unknown",
                    "message": msg
                })
                    .to_string()
            }
        };

        if tx.send(Message::text(formatted_msg)).await.is_err() {
            break;
        }
    }
}
