use error_stack::Report;
use sonata_openapi::apis::ErrorHandler;

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
enum ServerError {
    /// Internal error occurred
    InternalError(String),
}

impl ErrorHandler<Report<ServerError>> for Server {}

pub struct Server {}
