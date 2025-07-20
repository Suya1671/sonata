use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ChangePasswordResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetUserResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUsersResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetUsersResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostChangePasswordResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreateUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeleteUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetUserResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetUsersResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetUsersResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUpdateUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateUserResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// UserManagement
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait UserManagement<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Changes the password of an existing user on the server..
    ///
    /// ChangePassword - GET /rest/changePassword
    async fn change_password(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::ChangePasswordQueryParams,
    ) -> Result<ChangePasswordResponse, E>;

    /// Creates a new user on the server..
    ///
    /// CreateUser - GET /rest/createUser
    async fn create_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateUserQueryParams,
    ) -> Result<CreateUserResponse, E>;

    /// Deletes an existing user on the server..
    ///
    /// DeleteUser - GET /rest/deleteUser
    async fn delete_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteUserQueryParams,
    ) -> Result<DeleteUserResponse, E>;

    /// Get details about a given user, including which authorization roles and folder access it has..
    ///
    /// GetUser - GET /rest/getUser
    async fn get_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetUserQueryParams,
    ) -> Result<GetUserResponse, E>;

    /// Get details about all users, including which authorization roles and folder access they have.
    ///
    /// GetUsers - GET /rest/getUsers
    async fn get_users(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetUsersResponse, E>;

    /// Changes the password of an existing user on the server..
    ///
    /// PostChangePassword - POST /rest/changePassword
    async fn post_change_password(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostChangePasswordRequest,
    ) -> Result<PostChangePasswordResponse, E>;

    /// Creates a new user on the server..
    ///
    /// PostCreateUser - POST /rest/createUser
    async fn post_create_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateUserRequest,
    ) -> Result<PostCreateUserResponse, E>;

    /// Deletes an existing user on the server..
    ///
    /// PostDeleteUser - POST /rest/deleteUser
    async fn post_delete_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteUserRequest,
    ) -> Result<PostDeleteUserResponse, E>;

    /// Get details about a given user, including which authorization roles and folder access it has..
    ///
    /// PostGetUser - POST /rest/getUser
    async fn post_get_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetUserRequest,
    ) -> Result<PostGetUserResponse, E>;

    /// Get details about all users, including which authorization roles and folder access they have.
    ///
    /// PostGetUsers - POST /rest/getUsers
    async fn post_get_users(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetUsersResponse, E>;

    /// Modifies an existing user on the server..
    ///
    /// PostUpdateUser - POST /rest/updateUser
    async fn post_update_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateUserRequest,
    ) -> Result<PostUpdateUserResponse, E>;

    /// Modifies an existing user on the server..
    ///
    /// UpdateUser - GET /rest/updateUser
    async fn update_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateUserQueryParams,
    ) -> Result<UpdateUserResponse, E>;
}
