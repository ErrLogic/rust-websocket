use crate::errors::ErrorMessage;
use crate::state::ServerState;
use crate::utils::subscribe_channel;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use warp::reply::json;
use warp::{http::StatusCode, reject, ws::WebSocket, Rejection, Reply};

pub async fn handle_ws(ws: WebSocket, state: ServerState) {
    let (tx, mut rx) = ws.split();
    let tx = Arc::new(Mutex::new(tx));

    while let Some(Ok(msg)) = rx.next().await {
        if let Ok(text) = msg.to_str() {
            let mut channels = state.channels.lock().await;
            if let Some(sender) = channels.get(text) {
                let receiver = sender.subscribe();
                tokio::spawn(subscribe_channel(tx.clone(), receiver));
            } else {
                let (sender, _) = broadcast::channel(100);
                channels.insert(text.to_string(), sender.clone());
                tokio::spawn(subscribe_channel(tx.clone(), sender.subscribe()));
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub sender: String,
    pub channel: String,
    pub message: Value,
}

pub async fn handle_send(body: SendMessage, state: ServerState) -> Result<impl Reply, Rejection> {
    let channels = state.channels.lock().await;

    if let Some(sender) = channels.get(&body.channel) {
        let message_string = body.message.to_string();

        let formatted_message = json!({
            "sender": body.sender,
            "message": message_string
        });

        if sender.send(formatted_message.to_string()).is_ok() {
            let response = json!({
                "status": "sent",
                "channel": body.channel,
                "data": {
                    "message": message_string,
                    "sender": body.sender
                }
            });

            return Ok(warp::reply::with_status(json(&response), StatusCode::OK));
        }
    }

    Err(reject::custom(ErrorMessage {
        message: "Channel not found".to_string(),
    }))
}

