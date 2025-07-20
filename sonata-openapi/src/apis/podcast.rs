use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreatePodcastChannelResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeletePodcastChannelResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeletePodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DownloadPodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetNewestPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetNewestPodcastsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPodcastsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreatePodcastChannelResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeletePodcastChannelResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeletePodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDownloadPodcastEpisodeResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetNewestPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetNewestPodcastsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPodcastsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostRefreshPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RefreshPodcastsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// Podcast
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Podcast<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Adds a new Podcast channel..
    ///
    /// CreatePodcastChannel - GET /rest/createPodcastChannel
    async fn create_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreatePodcastChannelQueryParams,
    ) -> Result<CreatePodcastChannelResponse, E>;

    /// Deletes a Podcast channel..
    ///
    /// DeletePodcastChannel - GET /rest/deletePodcastChannel
    async fn delete_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePodcastChannelQueryParams,
    ) -> Result<DeletePodcastChannelResponse, E>;

    /// Deletes a Podcast episode..
    ///
    /// DeletePodcastEpisode - GET /rest/deletePodcastEpisode
    async fn delete_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePodcastEpisodeQueryParams,
    ) -> Result<DeletePodcastEpisodeResponse, E>;

    /// Request the server to start downloading a given Podcast episode..
    ///
    /// DownloadPodcastEpisode - GET /rest/downloadPodcastEpisode
    async fn download_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DownloadPodcastEpisodeQueryParams,
    ) -> Result<DownloadPodcastEpisodeResponse, E>;

    /// Returns the most recently published Podcast episodes..
    ///
    /// GetNewestPodcasts - GET /rest/getNewestPodcasts
    async fn get_newest_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetNewestPodcastsQueryParams,
    ) -> Result<GetNewestPodcastsResponse, E>;

    /// Returns all Podcast channels the server subscribes to, and (optionally) their episodes..
    ///
    /// GetPodcasts - GET /rest/getPodcasts
    async fn get_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPodcastsQueryParams,
    ) -> Result<GetPodcastsResponse, E>;

    /// Adds a new Podcast channel..
    ///
    /// PostCreatePodcastChannel - POST /rest/createPodcastChannel
    async fn post_create_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreatePodcastChannelRequest,
    ) -> Result<PostCreatePodcastChannelResponse, E>;

    /// Deletes a Podcast channel..
    ///
    /// PostDeletePodcastChannel - POST /rest/deletePodcastChannel
    async fn post_delete_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePodcastChannelRequest,
    ) -> Result<PostDeletePodcastChannelResponse, E>;

    /// Deletes a Podcast episode..
    ///
    /// PostDeletePodcastEpisode - POST /rest/deletePodcastEpisode
    async fn post_delete_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePodcastEpisodeRequest,
    ) -> Result<PostDeletePodcastEpisodeResponse, E>;

    /// Request the server to start downloading a given Podcast episode..
    ///
    /// PostDownloadPodcastEpisode - POST /rest/downloadPodcastEpisode
    async fn post_download_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDownloadPodcastEpisodeRequest,
    ) -> Result<PostDownloadPodcastEpisodeResponse, E>;

    /// Returns the most recently published Podcast episodes..
    ///
    /// PostGetNewestPodcasts - POST /rest/getNewestPodcasts
    async fn post_get_newest_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetNewestPodcastsRequest>,
    ) -> Result<PostGetNewestPodcastsResponse, E>;

    /// Returns all Podcast channels the server subscribes to, and (optionally) their episodes..
    ///
    /// PostGetPodcasts - POST /rest/getPodcasts
    async fn post_get_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetPodcastsRequest>,
    ) -> Result<PostGetPodcastsResponse, E>;

    /// Requests the server to check for new Podcast episodes..
    ///
    /// PostRefreshPodcasts - POST /rest/refreshPodcasts
    async fn post_refresh_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostRefreshPodcastsResponse, E>;

    /// Requests the server to check for new Podcast episodes..
    ///
    /// RefreshPodcasts - GET /rest/refreshPodcasts
    async fn refresh_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<RefreshPodcastsResponse, E>;
}
