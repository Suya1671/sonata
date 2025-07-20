#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::chat::{
        Chat, GetAddChatMessageResponse, GetChatMessagesResponse, PostAddChatMessageResponse,
        PostGetChatMessagesResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Chat<Report<ServerError>> for Server {
    async fn get_add_chat_message(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAddChatMessageQueryParams,
    ) -> Result<GetAddChatMessageResponse, ServerError> {
        todo!()
    }

    async fn get_chat_messages(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetChatMessagesResponse, ServerError> {
        todo!()
    }

    async fn post_add_chat_message(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostAddChatMessageRequest,
    ) -> Result<PostAddChatMessageResponse, ServerError> {
        todo!()
    }

    async fn post_get_chat_messages(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetChatMessagesResponse, ServerError> {
        todo!()
    }
}
