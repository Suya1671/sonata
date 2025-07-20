use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetOpenSubsonicExtensionsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetOpenSubsonicExtensionsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetOpenSubsonicExtensionsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetOpenSubsonicExtensionsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// AdditionSystem
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait AdditionSystem<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// List the OpenSubsonic extensions supported by this server..
    ///
    /// GetOpenSubsonicExtensions - GET /rest/getOpenSubsonicExtensions
    async fn get_open_subsonic_extensions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetOpenSubsonicExtensionsResponse, E>;

    /// List the OpenSubsonic extensions supported by this server..
    ///
    /// PostGetOpenSubsonicExtensions - POST /rest/getOpenSubsonicExtensions
    async fn post_get_open_subsonic_extensions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetOpenSubsonicExtensionsResponse, E>;
}
