use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetAddChatMessageResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetChatMessagesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetChatMessagesResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostAddChatMessageResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetChatMessagesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetChatMessagesResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

/// Chat
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Chat<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Adds a message to the chat log..
    ///
    /// GetAddChatMessage - GET /rest/addChatMessage
    async fn get_add_chat_message(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAddChatMessageQueryParams,
    ) -> Result<GetAddChatMessageResponse, E>;

    /// Returns the current visible (non-expired) chat messages..
    ///
    /// GetChatMessages - GET /rest/getChatMessages
    async fn get_chat_messages(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetChatMessagesResponse, E>;

    /// Adds a message to the chat log..
    ///
    /// PostAddChatMessage - POST /rest/addChatMessage
    async fn post_add_chat_message(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostAddChatMessageRequest,
    ) -> Result<PostAddChatMessageResponse, E>;

    /// Returns the current visible (non-expired) chat messages..
    ///
    /// PostGetChatMessages - POST /rest/getChatMessages
    async fn post_get_chat_messages(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetChatMessagesResponse, E>;
}
