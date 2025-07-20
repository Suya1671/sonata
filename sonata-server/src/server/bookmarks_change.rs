#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::bookmarks_change::{
        BookmarksChange, PostSavePlayQueueByIndexResponse, PostSavePlayQueueResponse,
        SavePlayQueueByIndexResponse, SavePlayQueueResponse,
    },
    models,
};

use super::{Server, ServerError};

impl BookmarksChange<Report<ServerError>> for Server {
    async fn post_save_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostSavePlayQueueRequest>,
    ) -> Result<PostSavePlayQueueResponse, ServerError> {
        todo!()
    }

    async fn post_save_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostSavePlayQueueRequest>,
    ) -> Result<PostSavePlayQueueByIndexResponse, ServerError> {
        todo!()
    }

    async fn save_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SavePlayQueueQueryParams,
    ) -> Result<SavePlayQueueResponse, ServerError> {
        todo!()
    }

    async fn save_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SavePlayQueueByIndexQueryParams,
    ) -> Result<SavePlayQueueByIndexResponse, ServerError> {
        todo!()
    }
}
