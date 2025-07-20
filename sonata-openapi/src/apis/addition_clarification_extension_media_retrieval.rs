use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostStreamResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StreamResponse {
    /// Success (binary) or error (xml) response
    Status200_Success(ByteArray),
}

/// AdditionClarificationExtensionMediaRetrieval
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait AdditionClarificationExtensionMediaRetrieval<
    E: std::fmt::Debug + Send + Sync + 'static = (),
>: super::ErrorHandler<E>
{
    /// Streams a given media file..
    ///
    /// PostStream - POST /rest/stream
    async fn post_stream(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostStreamRequest,
    ) -> Result<PostStreamResponse, E>;

    /// Streams a given media file..
    ///
    /// Stream - GET /rest/stream
    async fn stream(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::StreamQueryParams,
    ) -> Result<StreamResponse, E>;
}
