use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetScanStatusResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetScanStatusResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetScanStatusResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetScanStatusResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostStartScanResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::StartScanResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StartScanResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::StartScanResponse),
}

/// MediaLibraryScanning
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait MediaLibraryScanning<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns the current status for media library scanning..
    ///
    /// GetScanStatus - GET /rest/getScanStatus
    async fn get_scan_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetScanStatusResponse, E>;

    /// Returns the current status for media library scanning..
    ///
    /// PostGetScanStatus - POST /rest/getScanStatus
    async fn post_get_scan_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetScanStatusResponse, E>;

    /// Initiates a rescan of the media libraries..
    ///
    /// PostStartScan - POST /rest/startScan
    async fn post_start_scan(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostStartScanResponse, E>;

    /// Initiates a rescan of the media libraries..
    ///
    /// StartScan - GET /rest/startScan
    async fn start_scan(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<StartScanResponse, E>;
}
