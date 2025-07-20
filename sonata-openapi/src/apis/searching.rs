use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSearchResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SearchResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostSearch2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::Search2Response),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SearchResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SearchResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum Search2Response {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::Search2Response),
}

/// Searching
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Searching<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns a listing of files matching the given search criteria. Supports paging through the result..
    ///
    /// PostSearch - POST /rest/search
    async fn post_search(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearchRequest,
    ) -> Result<PostSearchResponse, E>;

    /// Returns a listing of files matching the given search criteria. Supports paging through the result. (v2).
    ///
    /// PostSearch2 - POST /rest/search2
    async fn post_search2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearch2Request,
    ) -> Result<PostSearch2Response, E>;

    /// Returns a listing of files matching the given search criteria. Supports paging through the result..
    ///
    /// Search - GET /rest/search
    async fn search(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SearchQueryParams,
    ) -> Result<SearchResponse, E>;

    /// Returns a listing of files matching the given search criteria. Supports paging through the result. (v2).
    ///
    /// Search2 - GET /rest/search2
    async fn search2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::Search2QueryParams,
    ) -> Result<Search2Response, E>;
}
