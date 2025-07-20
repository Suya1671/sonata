use std::sync::Arc;

use axum::extract::Request;
use error_stack::{Report, Result, ResultExt};
use sonata_openapi::{apis::ErrorHandler, server};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use tracing::{Level, info};

mod addition_clarification_extension_media_retrieval;
mod addition_extension_system;
mod addition_system;
mod bookmarks;
mod bookmarks_change;
mod browsing;
mod chat;
mod clarification_searching;
mod extension_media_retrieval;
mod extension_podcast;
mod internet_radio;
mod jukebox;
mod lists;
mod media_annotation;
mod media_library_scanning;
mod media_retrieval;
mod playlists;
mod podcast;
mod searching;
mod sharing;
mod system;
mod user_management;

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum ServerError {
    /// Internal error occurred
    InternalError(String),
    /// Error occurred while running the server
    AxumServerError,
}

impl ErrorHandler<Report<ServerError>> for Server {}

pub struct Server {}

#[tracing::instrument(skip(addr))]
pub async fn server_task(addr: &str) -> Result<(), ServerError> {
    let app = server::new(Arc::new(Server {}));

    let app = app.layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            tracing::span!(
                Level::DEBUG,
                "request",
                method = ?request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        }),
    );

    // Run the server with graceful shutdown
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Starting server");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .change_context(ServerError::AxumServerError)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
