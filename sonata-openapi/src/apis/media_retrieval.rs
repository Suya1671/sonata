use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DownloadResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAvatarResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCaptionsResponse {
    /// Returns the raw video captions.
    Status200_ReturnsTheRawVideoCaptions(ByteArray),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCoverArtResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetLyricsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLyricsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum HlsPeriodM3u8Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDownloadResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetAvatarResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetCaptionsResponse {
    /// Returns the raw video captions.
    Status200_ReturnsTheRawVideoCaptions(ByteArray),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetCoverArtResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetLyricsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLyricsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostHlsPeriodM3u8Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(String),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// MediaRetrieval
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait MediaRetrieval<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Downloads a given media file..
    ///
    /// Download - GET /rest/download
    async fn download(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DownloadQueryParams,
    ) -> Result<DownloadResponse, E>;

    /// Returns the avatar (personal image) for a user..
    ///
    /// GetAvatar - GET /rest/getAvatar
    async fn get_avatar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAvatarQueryParams,
    ) -> Result<GetAvatarResponse, E>;

    /// Returns captions (subtitles) for a video..
    ///
    /// GetCaptions - GET /rest/getCaptions
    async fn get_captions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetCaptionsQueryParams,
    ) -> Result<GetCaptionsResponse, E>;

    /// Returns a cover art image..
    ///
    /// GetCoverArt - GET /rest/getCoverArt
    async fn get_cover_art(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetCoverArtQueryParams,
    ) -> Result<GetCoverArtResponse, E>;

    /// Searches for and returns lyrics for a given song..
    ///
    /// GetLyrics - GET /rest/getLyrics
    async fn get_lyrics(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetLyricsQueryParams,
    ) -> Result<GetLyricsResponse, E>;

    /// Downloads a given media file (HLS)..
    ///
    /// HlsPeriodM3u8 - GET /rest/hls.m3u8
    async fn hls_period_m3u8(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::HlsPeriodM3u8QueryParams,
    ) -> Result<HlsPeriodM3u8Response, E>;

    /// Downloads a given media file..
    ///
    /// PostDownload - POST /rest/download
    async fn post_download(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDownloadRequest,
    ) -> Result<PostDownloadResponse, E>;

    /// Returns the avatar (personal image) for a user..
    ///
    /// PostGetAvatar - POST /rest/getAvatar
    async fn post_get_avatar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAvatarRequest,
    ) -> Result<PostGetAvatarResponse, E>;

    /// Returns captions (subtitles) for a video..
    ///
    /// PostGetCaptions - POST /rest/getCaptions
    async fn post_get_captions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetCaptionsRequest,
    ) -> Result<PostGetCaptionsResponse, E>;

    /// Returns a cover art image..
    ///
    /// PostGetCoverArt - POST /rest/getCoverArt
    async fn post_get_cover_art(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetCoverArtRequest,
    ) -> Result<PostGetCoverArtResponse, E>;

    /// Searches for and returns lyrics for a given song..
    ///
    /// PostGetLyrics - POST /rest/getLyrics
    async fn post_get_lyrics(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetLyricsRequest>,
    ) -> Result<PostGetLyricsResponse, E>;

    /// Downloads a given media file (HLS)..
    ///
    /// PostHlsPeriodM3u8 - POST /rest/hls.m3u8
    async fn post_hls_period_m3u8(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostHlsM3u8Request,
    ) -> Result<PostHlsPeriodM3u8Response, E>;
}
