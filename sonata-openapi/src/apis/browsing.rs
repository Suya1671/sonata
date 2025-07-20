use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAlbumResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAlbumInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumInfoResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAlbumInfo2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumInfoResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetArtistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetArtistInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistInfoResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetArtistInfo2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistInfo2Response),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetArtistsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetGenresResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetGenresResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetIndexesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetIndexesResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetMusicDirectoryResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetMusicDirectoryResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetMusicFoldersResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetMusicFoldersResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSimilarSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSimilarSongsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSimilarSongs2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSimilarSongs2Response),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSongResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSongResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTopSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetTopSongsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetVideoInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetVideoInfoResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetVideosResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetVideosResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAlbumResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAlbumInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumInfoResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAlbumInfo2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetAlbumInfoResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetArtistResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetArtistInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistInfoResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetArtistInfo2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistInfo2Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetArtistsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetArtistsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetGenresResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetGenresResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetIndexesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetIndexesResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetMusicDirectoryResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetMusicDirectoryResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetMusicFoldersResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetMusicFoldersResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetSimilarSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSimilarSongsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetSimilarSongs2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSimilarSongs2Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetSongResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSongResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetTopSongsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetTopSongsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetVideoInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetVideoInfoResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetVideosResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetVideosResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// Browsing
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Browsing<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns details for an album..
    ///
    /// GetAlbum - GET /rest/getAlbum
    async fn get_album(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumQueryParams,
    ) -> Result<GetAlbumResponse, E>;

    /// Returns album info..
    ///
    /// GetAlbumInfo - GET /rest/getAlbumInfo
    async fn get_album_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumInfoQueryParams,
    ) -> Result<GetAlbumInfoResponse, E>;

    /// Returns album info (v2)..
    ///
    /// GetAlbumInfo2 - GET /rest/getAlbumInfo2
    async fn get_album_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumInfo2QueryParams,
    ) -> Result<GetAlbumInfo2Response, E>;

    /// Returns details for an artist..
    ///
    /// GetArtist - GET /rest/getArtist
    async fn get_artist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistQueryParams,
    ) -> Result<GetArtistResponse, E>;

    /// Returns artist info..
    ///
    /// GetArtistInfo - GET /rest/getArtistInfo
    async fn get_artist_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistInfoQueryParams,
    ) -> Result<GetArtistInfoResponse, E>;

    /// Returns artist info (v2)..
    ///
    /// GetArtistInfo2 - GET /rest/getArtistInfo2
    async fn get_artist_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistInfo2QueryParams,
    ) -> Result<GetArtistInfo2Response, E>;

    /// Returns all artists..
    ///
    /// GetArtists - GET /rest/getArtists
    async fn get_artists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistsQueryParams,
    ) -> Result<GetArtistsResponse, E>;

    /// Returns all genres..
    ///
    /// GetGenres - GET /rest/getGenres
    async fn get_genres(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetGenresResponse, E>;

    /// Returns an indexed structure of all artists..
    ///
    /// GetIndexes - GET /rest/getIndexes
    async fn get_indexes(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetIndexesQueryParams,
    ) -> Result<GetIndexesResponse, E>;

    /// Returns a listing of all files in a music directory..
    ///
    /// GetMusicDirectory - GET /rest/getMusicDirectory
    async fn get_music_directory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetMusicDirectoryQueryParams,
    ) -> Result<GetMusicDirectoryResponse, E>;

    /// Returns all configured top-level music folders..
    ///
    /// GetMusicFolders - GET /rest/getMusicFolders
    async fn get_music_folders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetMusicFoldersResponse, E>;

    /// Returns a random collection of songs from the given artist and similar artists..
    ///
    /// GetSimilarSongs - GET /rest/getSimilarSongs
    async fn get_similar_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSimilarSongsQueryParams,
    ) -> Result<GetSimilarSongsResponse, E>;

    /// Returns a random collection of songs from the given artist and similar artists (v2)..
    ///
    /// GetSimilarSongs2 - GET /rest/getSimilarSongs2
    async fn get_similar_songs2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSimilarSongs2QueryParams,
    ) -> Result<GetSimilarSongs2Response, E>;

    /// Returns details for a song..
    ///
    /// GetSong - GET /rest/getSong
    async fn get_song(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSongQueryParams,
    ) -> Result<GetSongResponse, E>;

    /// Returns top songs for the given artist..
    ///
    /// GetTopSongs - GET /rest/getTopSongs
    async fn get_top_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetTopSongsQueryParams,
    ) -> Result<GetTopSongsResponse, E>;

    /// Returns details for a video..
    ///
    /// GetVideoInfo - GET /rest/getVideoInfo
    async fn get_video_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetVideoInfoQueryParams,
    ) -> Result<GetVideoInfoResponse, E>;

    /// Returns all video files..
    ///
    /// GetVideos - GET /rest/getVideos
    async fn get_videos(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetVideosResponse, E>;

    /// Returns details for an album..
    ///
    /// PostGetAlbum - POST /rest/getAlbum
    async fn post_get_album(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumRequest,
    ) -> Result<PostGetAlbumResponse, E>;

    /// Returns album info..
    ///
    /// PostGetAlbumInfo - POST /rest/getAlbumInfo
    async fn post_get_album_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumInfoRequest,
    ) -> Result<PostGetAlbumInfoResponse, E>;

    /// Returns album info (v2)..
    ///
    /// PostGetAlbumInfo2 - POST /rest/getAlbumInfo2
    async fn post_get_album_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumInfoRequest,
    ) -> Result<PostGetAlbumInfo2Response, E>;

    /// Returns details for an artist..
    ///
    /// PostGetArtist - POST /rest/getArtist
    async fn post_get_artist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistRequest,
    ) -> Result<PostGetArtistResponse, E>;

    /// Returns artist info..
    ///
    /// PostGetArtistInfo - POST /rest/getArtistInfo
    async fn post_get_artist_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistInfoRequest,
    ) -> Result<PostGetArtistInfoResponse, E>;

    /// Returns artist info (v2)..
    ///
    /// PostGetArtistInfo2 - POST /rest/getArtistInfo2
    async fn post_get_artist_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistInfoRequest,
    ) -> Result<PostGetArtistInfo2Response, E>;

    /// Returns all artists..
    ///
    /// PostGetArtists - POST /rest/getArtists
    async fn post_get_artists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetArtistsRequest>,
    ) -> Result<PostGetArtistsResponse, E>;

    /// Returns all genres..
    ///
    /// PostGetGenres - POST /rest/getGenres
    async fn post_get_genres(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetGenresResponse, E>;

    /// Returns an indexed structure of all artists..
    ///
    /// PostGetIndexes - POST /rest/getIndexes
    async fn post_get_indexes(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetIndexesRequest>,
    ) -> Result<PostGetIndexesResponse, E>;

    /// Returns a listing of all files in a music directory..
    ///
    /// PostGetMusicDirectory - POST /rest/getMusicDirectory
    async fn post_get_music_directory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetMusicDirectoryRequest,
    ) -> Result<PostGetMusicDirectoryResponse, E>;

    /// Returns all configured top-level music folders..
    ///
    /// PostGetMusicFolders - POST /rest/getMusicFolders
    async fn post_get_music_folders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetMusicFoldersResponse, E>;

    /// Returns a random collection of songs from the given artist and similar artists..
    ///
    /// PostGetSimilarSongs - POST /rest/getSimilarSongs
    async fn post_get_similar_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSimilarSongsRequest,
    ) -> Result<PostGetSimilarSongsResponse, E>;

    /// Returns a random collection of songs from the given artist and similar artists (v2)..
    ///
    /// PostGetSimilarSongs2 - POST /rest/getSimilarSongs2
    async fn post_get_similar_songs2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSimilarSongsRequest,
    ) -> Result<PostGetSimilarSongs2Response, E>;

    /// Returns details for a song..
    ///
    /// PostGetSong - POST /rest/getSong
    async fn post_get_song(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSongRequest,
    ) -> Result<PostGetSongResponse, E>;

    /// Returns top songs for the given artist..
    ///
    /// PostGetTopSongs - POST /rest/getTopSongs
    async fn post_get_top_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetTopSongsRequest,
    ) -> Result<PostGetTopSongsResponse, E>;

    /// Returns details for a video..
    ///
    /// PostGetVideoInfo - POST /rest/getVideoInfo
    async fn post_get_video_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetVideoInfoRequest,
    ) -> Result<PostGetVideoInfoResponse, E>;

    /// Returns all video files..
    ///
    /// PostGetVideos - POST /rest/getVideos
    async fn post_get_videos(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetVideosResponse, E>;
}
