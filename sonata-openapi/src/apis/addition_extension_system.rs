use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostTokenInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetTokenInfoResponse),
    /// Extension not supported
    Status404_ExtensionNotSupported,
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TokenInfoResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetTokenInfoResponse),
    /// Extension not supported
    Status404_ExtensionNotSupported,
}

/// AdditionExtensionSystem
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait AdditionExtensionSystem<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Returns information about an API key.
    ///
    /// PostTokenInfo - POST /rest/tokenInfo
    async fn post_token_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostTokenInfoResponse, E>;

    /// Returns information about an API key.
    ///
    /// TokenInfo - GET /rest/tokenInfo
    async fn token_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<TokenInfoResponse, E>;
}
