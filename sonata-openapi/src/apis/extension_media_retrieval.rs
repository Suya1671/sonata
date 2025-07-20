use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetLyricsBySongIdResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLyricsBySongIdResponse),
    /// Extension not supported.
    Status404_ExtensionNotSupported,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetLyricsBySongIdResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLyricsBySongIdResponse),
    /// Extension not supported.
    Status404_ExtensionNotSupported,
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// ExtensionMediaRetrieval
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait ExtensionMediaRetrieval<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Add support for synchronized lyrics, multiple languages, and retrieval by song ID. .
    ///
    /// GetLyricsBySongId - GET /rest/getLyricsBySongId
    async fn get_lyrics_by_song_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetLyricsBySongIdQueryParams,
    ) -> Result<GetLyricsBySongIdResponse, E>;

    /// Add support for synchronized lyrics, multiple languages, and retrieval by song ID..
    ///
    /// PostGetLyricsBySongId - POST /rest/getLyricsBySongId
    async fn post_get_lyrics_by_song_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetLyricsBySongIdRequest,
    ) -> Result<PostGetLyricsBySongIdResponse, E>;
}
