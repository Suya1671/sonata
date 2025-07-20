#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::sharing::{
        CreateShareResponse, DeleteShareResponse, GetSharesResponse, PostCreateShareResponse,
        PostDeleteShareResponse, PostGetSharesResponse, PostUpdateShareResponse, Sharing,
        UpdateShareResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Sharing<Report<ServerError>> for Server {
    async fn create_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateShareQueryParams,
    ) -> Result<CreateShareResponse, ServerError> {
        todo!()
    }

    async fn delete_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteShareQueryParams,
    ) -> Result<DeleteShareResponse, ServerError> {
        todo!()
    }

    async fn get_shares(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetSharesResponse, ServerError> {
        todo!()
    }

    async fn post_create_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateShareRequest,
    ) -> Result<PostCreateShareResponse, ServerError> {
        todo!()
    }

    async fn post_delete_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteShareRequest,
    ) -> Result<PostDeleteShareResponse, ServerError> {
        todo!()
    }

    async fn post_get_shares(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetSharesResponse, ServerError> {
        todo!()
    }

    async fn post_update_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateShareRequest,
    ) -> Result<PostUpdateShareResponse, ServerError> {
        todo!()
    }

    async fn update_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateShareQueryParams,
    ) -> Result<UpdateShareResponse, ServerError> {
        todo!()
    }
}
