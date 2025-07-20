use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAlbumListResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumListResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAlbumList2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumList2Response),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetNowPlayingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetNowPlayingResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetRandomSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetRandomSongsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSongsByGenreResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSongsByGenreResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetStarredResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetStarredResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetStarred2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetStarred2Response),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAlbumListResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumListResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAlbumList2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumList2Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetNowPlayingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetNowPlayingResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetRandomSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetRandomSongsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetSongsByGenreResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSongsByGenreResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetStarredResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetStarredResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetStarred2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetStarred2Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// Lists
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Lists<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Returns a list of random, newest, highest rated etc. albums..
    ///
    /// GetAlbumList - GET /rest/getAlbumList
    async fn get_album_list(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumListQueryParams,
    ) -> Result<GetAlbumListResponse, E>;

    /// Returns a list of random, newest, highest rated etc. albums (v2)..
    ///
    /// GetAlbumList2 - GET /rest/getAlbumList2
    async fn get_album_list2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumList2QueryParams,
    ) -> Result<GetAlbumList2Response, E>;

    /// Returns what is currently being played by all users..
    ///
    /// GetNowPlaying - GET /rest/getNowPlaying
    async fn get_now_playing(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetNowPlayingResponse, E>;

    /// Returns random songs matching the given criteria..
    ///
    /// GetRandomSongs - GET /rest/getRandomSongs
    async fn get_random_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetRandomSongsQueryParams,
    ) -> Result<GetRandomSongsResponse, E>;

    /// Returns songs in a given genre..
    ///
    /// GetSongsByGenre - GET /rest/getSongsByGenre
    async fn get_songs_by_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSongsByGenreQueryParams,
    ) -> Result<GetSongsByGenreResponse, E>;

    /// Returns starred songs, albums and artists..
    ///
    /// GetStarred - GET /rest/getStarred
    async fn get_starred(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetStarredQueryParams,
    ) -> Result<GetStarredResponse, E>;

    /// Returns starred songs, albums and artists..
    ///
    /// GetStarred2 - GET /rest/getStarred2
    async fn get_starred2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetStarred2QueryParams,
    ) -> Result<GetStarred2Response, E>;

    /// Returns a list of random, newest, highest rated etc. albums..
    ///
    /// PostGetAlbumList - POST /rest/getAlbumList
    async fn post_get_album_list(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumListRequest,
    ) -> Result<PostGetAlbumListResponse, E>;

    /// Returns a list of random, newest, highest rated etc. albums (v2)..
    ///
    /// PostGetAlbumList2 - POST /rest/getAlbumList2
    async fn post_get_album_list2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumListRequest,
    ) -> Result<PostGetAlbumList2Response, E>;

    /// Returns what is currently being played by all users..
    ///
    /// PostGetNowPlaying - POST /rest/getNowPlaying
    async fn post_get_now_playing(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetNowPlayingResponse, E>;

    /// Returns random songs matching the given criteria..
    ///
    /// PostGetRandomSongs - POST /rest/getRandomSongs
    async fn post_get_random_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetRandomSongsRequest>,
    ) -> Result<PostGetRandomSongsResponse, E>;

    /// Returns songs in a given genre..
    ///
    /// PostGetSongsByGenre - POST /rest/getSongsByGenre
    async fn post_get_songs_by_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSongsByGenreRequest,
    ) -> Result<PostGetSongsByGenreResponse, E>;

    /// Returns starred songs, albums and artists..
    ///
    /// PostGetStarred - POST /rest/getStarred
    async fn post_get_starred(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetStarredResponse, E>;

    /// Returns starred songs, albums and artists..
    ///
    /// PostGetStarred2 - POST /rest/getStarred2
    async fn post_get_starred2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetStarred2Response, E>;
}
