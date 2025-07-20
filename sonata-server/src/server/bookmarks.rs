use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::bookmarks::{
        Bookmarks, CreateBookmarkResponse, DeleteBookmarkResponse, GetBookmarksResponse,
        GetPlayQueueByIndexResponse, GetPlayQueueResponse, PostCreateBookmarkResponse,
        PostDeleteBookmarkResponse, PostGetBookmarksResponse, PostGetPlayQueueByIndexResponse,
        PostGetPlayQueueResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Bookmarks<Report<ServerError>> for Server {
    async fn create_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateBookmarkQueryParams,
    ) -> Result<CreateBookmarkResponse, ServerError> {
        todo!()
    }

    async fn delete_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteBookmarkQueryParams,
    ) -> Result<DeleteBookmarkResponse, ServerError> {
        todo!()
    }

    async fn get_bookmarks(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetBookmarksResponse, ServerError> {
        todo!()
    }

    async fn get_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetPlayQueueResponse, ServerError> {
        todo!()
    }

    async fn get_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetPlayQueueByIndexResponse, ServerError> {
        todo!()
    }

    async fn post_create_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateBookmarkRequest,
    ) -> Result<PostCreateBookmarkResponse, ServerError> {
        todo!()
    }

    async fn post_delete_bookmark(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteBookmarkRequest,
    ) -> Result<PostDeleteBookmarkResponse, ServerError> {
        todo!()
    }

    async fn post_get_bookmarks(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetBookmarksResponse, ServerError> {
        todo!()
    }

    async fn post_get_play_queue(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetPlayQueueResponse, ServerError> {
        todo!()
    }

    async fn post_get_play_queue_by_index(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetPlayQueueByIndexResponse, ServerError> {
        todo!()
    }
}
