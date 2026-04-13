use tokio::net::TcpListener;

/// Serve an Axum router, notifying the topcoat dev server once the
/// application is ready to accept connections.
///
/// This is a thin wrapper around [`axum::serve`] that calls
/// [`crate::dev::notify_ready`] before entering the accept loop.
pub async fn serve(
    listener: TcpListener,
    app: impl Into<axum::Router>,
) -> Result<(), std::io::Error> {
    crate::dev::notify_ready().await;
    axum::serve(listener, app.into()).await
}
