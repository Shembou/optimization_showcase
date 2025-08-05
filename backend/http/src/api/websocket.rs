use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tracing::info;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            info!("Received text message: {}", text);
            if socket
                .send(Message::Text(format!("Echo: {}", text).into()))
                .await
                .is_err()
            {
                info!("Client disconnected");
                return;
            }
        }
    }
}
