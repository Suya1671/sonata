use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum JukeboxControlResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::JukeboxControlResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostJukeboxControlResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::JukeboxControlResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// Jukebox
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Jukebox<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Controls the jukebox, i.e., playback directly on the server’s audio hardware..
    ///
    /// JukeboxControl - GET /rest/jukeboxControl
    async fn jukebox_control(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::JukeboxControlQueryParams,
    ) -> Result<JukeboxControlResponse, E>;

    /// Controls the jukebox, i.e., playback directly on the server’s audio hardware..
    ///
    /// PostJukeboxControl - POST /rest/jukeboxControl
    async fn post_jukebox_control(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostJukeboxControlRequest,
    ) -> Result<PostJukeboxControlResponse, E>;
}
