use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostScrobbleResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSetRatingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostStarResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUnstarResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ScrobbleResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SetRatingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StarResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UnstarResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// MediaAnnotation
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait MediaAnnotation<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Registers the local playback of one or more media files..
    ///
    /// PostScrobble - POST /rest/scrobble
    async fn post_scrobble(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostScrobbleRequest,
    ) -> Result<PostScrobbleResponse, E>;

    /// Sets the rating for a music file..
    ///
    /// PostSetRating - POST /rest/setRating
    async fn post_set_rating(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSetRatingRequest,
    ) -> Result<PostSetRatingResponse, E>;

    /// Attaches a star to a song, album or artist..
    ///
    /// PostStar - POST /rest/star
    async fn post_star(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostStarRequest,
    ) -> Result<PostStarResponse, E>;

    /// Removes a star to a song, album or artist..
    ///
    /// PostUnstar - POST /rest/unstar
    async fn post_unstar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUnstarRequest,
    ) -> Result<PostUnstarResponse, E>;

    /// Registers the local playback of one or more media files..
    ///
    /// Scrobble - GET /rest/scrobble
    async fn scrobble(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::ScrobbleQueryParams,
    ) -> Result<ScrobbleResponse, E>;

    /// Sets the rating for a music file..
    ///
    /// SetRating - GET /rest/setRating
    async fn set_rating(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SetRatingQueryParams,
    ) -> Result<SetRatingResponse, E>;

    /// Attaches a star to a song, album or artist..
    ///
    /// Star - GET /rest/star
    async fn star(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::StarQueryParams,
    ) -> Result<StarResponse, E>;

    /// Removes a star to a song, album or artist..
    ///
    /// Unstar - GET /rest/unstar
    async fn unstar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UnstarQueryParams,
    ) -> Result<UnstarResponse, E>;
}
