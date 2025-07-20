#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::searching::{
        PostSearch2Response, PostSearchResponse, Search2Response, SearchResponse, Searching,
    },
    models,
};

use super::{Server, ServerError};

impl Searching<Report<ServerError>> for Server {
    async fn post_search(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearchRequest,
    ) -> Result<PostSearchResponse, ServerError> {
        todo!()
    }

    async fn post_search2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearch2Request,
    ) -> Result<PostSearch2Response, ServerError> {
        todo!()
    }

    async fn search(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SearchQueryParams,
    ) -> Result<SearchResponse, ServerError> {
        todo!()
    }

    async fn search2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::Search2QueryParams,
    ) -> Result<Search2Response, ServerError> {
        todo!()
    }
}
