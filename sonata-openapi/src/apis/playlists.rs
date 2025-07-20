use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreatePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::CreatePlaylistResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeletePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlaylistResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPlaylistsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlaylistsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreatePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::CreatePlaylistResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeletePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlaylistResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetPlaylistsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetPlaylistsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUpdatePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePlaylistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// Playlists
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Playlists<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Creates (or updates) a playlist..
    ///
    /// CreatePlaylist - GET /rest/createPlaylist
    async fn create_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreatePlaylistQueryParams,
    ) -> Result<CreatePlaylistResponse, E>;

    /// Deletes a saved playlist..
    ///
    /// DeletePlaylist - GET /rest/deletePlaylist
    async fn delete_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePlaylistQueryParams,
    ) -> Result<DeletePlaylistResponse, E>;

    /// Returns a listing of files in a saved playlist..
    ///
    /// GetPlaylist - GET /rest/getPlaylist
    async fn get_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPlaylistQueryParams,
    ) -> Result<GetPlaylistResponse, E>;

    /// Returns all playlists a user is allowed to play..
    ///
    /// GetPlaylists - GET /rest/getPlaylists
    async fn get_playlists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPlaylistsQueryParams,
    ) -> Result<GetPlaylistsResponse, E>;

    /// Creates (or updates) a playlist..
    ///
    /// PostCreatePlaylist - POST /rest/createPlaylist
    async fn post_create_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreatePlaylistRequest,
    ) -> Result<PostCreatePlaylistResponse, E>;

    /// Deletes a saved playlist..
    ///
    /// PostDeletePlaylist - POST /rest/deletePlaylist
    async fn post_delete_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePlaylistRequest,
    ) -> Result<PostDeletePlaylistResponse, E>;

    /// Returns a listing of files in a saved playlist..
    ///
    /// PostGetPlaylist - POST /rest/getPlaylist
    async fn post_get_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetPlaylistRequest,
    ) -> Result<PostGetPlaylistResponse, E>;

    /// Returns all playlists a user is allowed to play..
    ///
    /// PostGetPlaylists - POST /rest/getPlaylists
    async fn post_get_playlists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetPlaylistsRequest>,
    ) -> Result<PostGetPlaylistsResponse, E>;

    /// Updates a playlist. Only the owner of a playlist is allowed to update it..
    ///
    /// PostUpdatePlaylist - POST /rest/updatePlaylist
    async fn post_update_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdatePlaylistRequest,
    ) -> Result<PostUpdatePlaylistResponse, E>;

    /// Updates a playlist. Only the owner of a playlist is allowed to update it..
    ///
    /// UpdatePlaylist - GET /rest/updatePlaylist
    async fn update_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdatePlaylistQueryParams,
    ) -> Result<UpdatePlaylistResponse, E>;
}
