use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateBookmarkResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteBookmarkResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetBookmarksResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetBookmarksResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPlayQueueResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlayQueueResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPlayQueueByIndexResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlayQueueByIndexResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreateBookmarkResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeleteBookmarkResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetBookmarksResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetBookmarksResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPlayQueueResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlayQueueResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPlayQueueByIndexResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlayQueueByIndexResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// Bookmarks
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Bookmarks<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Creates or updates a bookmark..
    ///
    /// CreateBookmark - GET /rest/createBookmark
    async fn create_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateBookmarkQueryParams,
    ) -> Result<CreateBookmarkResponse, E>;

    /// Deletes a bookmark..
    ///
    /// DeleteBookmark - GET /rest/deleteBookmark
    async fn delete_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteBookmarkQueryParams,
    ) -> Result<DeleteBookmarkResponse, E>;

    /// Returns all bookmarks for this user..
    ///
    /// GetBookmarks - GET /rest/getBookmarks
    async fn get_bookmarks(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetBookmarksResponse, E>;

    /// Returns the state of the play queue for this user..
    ///
    /// GetPlayQueue - GET /rest/getPlayQueue
    async fn get_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetPlayQueueResponse, E>;

    /// Returns the state of the play queue for this user, using queue index..
    ///
    /// GetPlayQueueByIndex - GET /rest/getPlayQueueByIndex
    async fn get_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetPlayQueueByIndexResponse, E>;

    /// Creates or updates a bookmark..
    ///
    /// PostCreateBookmark - POST /rest/createBookmark
    async fn post_create_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateBookmarkRequest,
    ) -> Result<PostCreateBookmarkResponse, E>;

    /// Deletes a bookmark..
    ///
    /// PostDeleteBookmark - POST /rest/deleteBookmark
    async fn post_delete_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteBookmarkRequest,
    ) -> Result<PostDeleteBookmarkResponse, E>;

    /// Returns all bookmarks for this user..
    ///
    /// PostGetBookmarks - POST /rest/getBookmarks
    async fn post_get_bookmarks(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetBookmarksResponse, E>;

    /// Returns the state of the play queue for this user..
    ///
    /// PostGetPlayQueue - POST /rest/getPlayQueue
    async fn post_get_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetPlayQueueResponse, E>;

    /// Returns the state of the play queue for this user..
    ///
    /// PostGetPlayQueueByIndex - POST /rest/getPlayQueueByIndex
    async fn post_get_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetPlayQueueByIndexResponse, E>;
}
