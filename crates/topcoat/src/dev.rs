use crate::Result;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use crate::view::{View, component, view};

/// Notify the topcoat dev server that the application is ready.
///
/// Connects to the dev server's WebSocket endpoint (derived from the
/// `TOPCOAT_DEV_URL` HTTP base URL provided by `topcoat dev`) and sends a
/// `"ready"` message. Does nothing if the env var is not set.
pub async fn notify_ready() {
    let Ok(base) = std::env::var("TOPCOAT_DEV_URL") else {
        return;
    };

    let ws_url = http_to_ws(&base) + "/ws";

    let Ok((mut ws, _)) = tokio_tungstenite::connect_async(&ws_url).await else {
        eprintln!("topcoat dev: failed to connect to {ws_url}");
        return;
    };

    let _ = ws.send(Message::Text("ready".into())).await;
    let _ = ws.close(None).await;
}

fn http_to_ws(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("http://") {
        format!("ws://{rest}")
    } else if let Some(rest) = url.strip_prefix("https://") {
        format!("wss://{rest}")
    } else {
        url.to_string()
    }
}

#[component]
#[expect(
    unused_variables,
    reason = "child is required by the component macro contract but unused here"
)]
pub async fn script(child: View) -> Result {
    let Ok(base) = std::env::var("TOPCOAT_DEV_URL") else {
        return view! {};
    };
    let src = format!("{base}/dev.js");

    view! {
        <script src=(src)></script>
    }
}
