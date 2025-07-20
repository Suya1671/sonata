use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSearch3Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::Search3Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum Search3Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::Search3Response),
}

/// ClarificationSearching
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait ClarificationSearching<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns albums, artists and songs matching the given search criteria. Supports paging through the result. (v3).
    ///
    /// PostSearch3 - POST /rest/search3
    async fn post_search3(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearch3Request,
    ) -> Result<PostSearch3Response, E>;

    /// Returns albums, artists and songs matching the given search criteria. Supports paging through the result. (v3).
    ///
    /// Search3 - GET /rest/search3
    async fn search3(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::Search3QueryParams,
    ) -> Result<Search3Response, E>;
}
