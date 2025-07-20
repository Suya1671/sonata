use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPodcastEpisodeResponse),
    /// Extension not supported.
    Status404_ExtensionNotSupported,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPodcastEpisodeResponse),
    /// Extension not supported.
    Status404_ExtensionNotSupported,
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// ExtensionPodcast
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait ExtensionPodcast<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns details for a podcast episode..
    ///
    /// GetPodcastEpisode - GET /rest/getPodcastEpisode
    async fn get_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPodcastEpisodeQueryParams,
    ) -> Result<GetPodcastEpisodeResponse, E>;

    /// Returns details for a podcast episode..
    ///
    /// PostGetPodcastEpisode - POST /rest/getPodcastEpisode
    async fn post_get_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetPodcastEpisodeRequest,
    ) -> Result<PostGetPodcastEpisodeResponse, E>;
}
