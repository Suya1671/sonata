use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetLicenseResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLicenseResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetLicenseResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetLicenseResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostPingResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// System
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait System<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Get details about the software license..
    ///
    /// GetLicense - GET /rest/getLicense
    async fn get_license(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetLicenseResponse, E>;

    /// Used to test connectivity with the server..
    ///
    /// Ping - GET /rest/ping
    async fn ping(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<PingResponse, E>;

    /// Get details about the software license..
    ///
    /// PostGetLicense - POST /rest/getLicense
    async fn post_get_license(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetLicenseResponse, E>;

    /// Used to test connectivity with the server..
    ///
    /// PostPing - POST /rest/ping
    async fn post_ping(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostPingResponse, E>;
}
