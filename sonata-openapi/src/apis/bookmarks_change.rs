use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSavePlayQueueResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSavePlayQueueByIndexResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SavePlayQueueResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SavePlayQueueByIndexResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// BookmarksChange
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait BookmarksChange<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Saves the state of the play queue for this user..
    ///
    /// PostSavePlayQueue - POST /rest/savePlayQueue
    async fn post_save_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostSavePlayQueueRequest>,
    ) -> Result<PostSavePlayQueueResponse, E>;

    /// Saves the state of the play queue for this user..
    ///
    /// PostSavePlayQueueByIndex - POST /rest/savePlayQueueByIndex
    async fn post_save_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostSavePlayQueueRequest>,
    ) -> Result<PostSavePlayQueueByIndexResponse, E>;

    /// Saves the state of the play queue for this user..
    ///
    /// SavePlayQueue - GET /rest/savePlayQueue
    async fn save_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SavePlayQueueQueryParams,
    ) -> Result<SavePlayQueueResponse, E>;

    /// Saves the state of the play queue for this user, using queue index..
    ///
    /// SavePlayQueueByIndex - GET /rest/savePlayQueueByIndex
    async fn save_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SavePlayQueueByIndexQueryParams,
    ) -> Result<SavePlayQueueByIndexResponse, E>;
}
