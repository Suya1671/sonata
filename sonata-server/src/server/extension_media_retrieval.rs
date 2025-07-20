#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::extension_media_retrieval::{
        ExtensionMediaRetrieval, GetLyricsBySongIdResponse, PostGetLyricsBySongIdResponse,
    },
    models,
};

use super::{Server, ServerError};

impl ExtensionMediaRetrieval<Report<ServerError>> for Server {
    async fn get_lyrics_by_song_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetLyricsBySongIdQueryParams,
    ) -> Result<GetLyricsBySongIdResponse, ServerError> {
        todo!()
    }

    async fn post_get_lyrics_by_song_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetLyricsBySongIdRequest,
    ) -> Result<PostGetLyricsBySongIdResponse, ServerError> {
        todo!()
    }
}
