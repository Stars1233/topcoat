use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

/// Run the WebSocket broadcast server.
///
/// Accepts connections on the given listener. When any client sends `"ready"`,
/// broadcasts `"reload"` to all other connected clients.
pub async fn run(listener: TcpListener) {
    let (tx, _) = broadcast::channel::<()>(16);
    let tx = Arc::new(tx);

    loop {
        let Ok((stream, _addr)) = listener.accept().await else {
            continue;
        };

        let Ok(ws) = tokio_tungstenite::accept_async(stream).await else {
            continue;
        };

        let tx = Arc::clone(&tx);
        tokio::spawn(handle_connection(ws, tx));
    }
}

async fn handle_connection(
    ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    tx: Arc<broadcast::Sender<()>>,
) {
    let (mut sink, mut stream) = ws.split();
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg = stream.next() => {
                let Some(Ok(msg)) = msg else { break };

                if let Message::Text(text) = msg
                    && text == "ready" {
                        let _ = tx.send(());
                    }
            }
            Ok(()) = rx.recv() => {
                if sink.send(Message::Text("reload".into())).await.is_err() {
                    break;
                }
            }
        }
    }
}
