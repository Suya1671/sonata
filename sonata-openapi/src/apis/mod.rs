pub mod addition_clarification_extension_media_retrieval;
pub mod addition_extension_system;
pub mod addition_system;
pub mod bookmarks;
pub mod bookmarks_change;
pub mod browsing;
pub mod chat;
pub mod clarification_searching;
pub mod extension_media_retrieval;
pub mod extension_podcast;
pub mod internet_radio;
pub mod jukebox;
pub mod lists;
pub mod media_annotation;
pub mod media_library_scanning;
pub mod media_retrieval;
pub mod playlists;
pub mod podcast;
pub mod searching;
pub mod sharing;
pub mod system;
pub mod user_management;

// Error handler for unhandled errors.
#[auto_async_send_sync::auto_send_sync]
pub trait ErrorHandler<E: std::fmt::Debug + Send + Sync + 'static = ()> {
    #[allow(unused_variables)]
    // #[tracing::instrument(skip_all)]
    async fn handle_error(
        &self,
        method: &::http::Method,
        host: &axum_extra::extract::Host,
        cookies: &axum_extra::extract::CookieJar,
        error: E,
    ) -> Result<axum::response::Response, http::StatusCode> {
        tracing::error!("Unhandled error: {:?}", error);
        axum::response::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::empty())
            .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
