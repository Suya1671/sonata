use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::CreateSharesResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetSharesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSharesResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreateShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::CreateSharesResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeleteShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetSharesResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetSharesResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUpdateShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateShareResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// Sharing
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait Sharing<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Creates a public URL that can be used by anyone to stream music or video from the server..
    ///
    /// CreateShare - GET /rest/createShare
    async fn create_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateShareQueryParams,
    ) -> Result<CreateShareResponse, E>;

    /// Deletes an existing share..
    ///
    /// DeleteShare - GET /rest/deleteShare
    async fn delete_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteShareQueryParams,
    ) -> Result<DeleteShareResponse, E>;

    /// Returns information about shared media this user is allowed to manage..
    ///
    /// GetShares - GET /rest/getShares
    async fn get_shares(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetSharesResponse, E>;

    /// Creates a public URL that can be used by anyone to stream music or video from the server..
    ///
    /// PostCreateShare - POST /rest/createShare
    async fn post_create_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateShareRequest,
    ) -> Result<PostCreateShareResponse, E>;

    /// Deletes an existing share..
    ///
    /// PostDeleteShare - POST /rest/deleteShare
    async fn post_delete_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteShareRequest,
    ) -> Result<PostDeleteShareResponse, E>;

    /// Returns information about shared media this user is allowed to manage..
    ///
    /// PostGetShares - POST /rest/getShares
    async fn post_get_shares(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetSharesResponse, E>;

    /// Updates the description and/or expiration date for an existing share..
    ///
    /// PostUpdateShare - POST /rest/updateShare
    async fn post_update_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateShareRequest,
    ) -> Result<PostUpdateShareResponse, E>;

    /// Updates the description and/or expiration date for an existing share..
    ///
    /// UpdateShare - GET /rest/updateShare
    async fn update_share(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateShareQueryParams,
    ) -> Result<UpdateShareResponse, E>;
}
