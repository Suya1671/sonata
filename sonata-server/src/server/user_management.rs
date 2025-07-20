#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::user_management::{
        ChangePasswordResponse, CreateUserResponse, DeleteUserResponse, GetUserResponse,
        GetUsersResponse, PostChangePasswordResponse, PostCreateUserResponse,
        PostDeleteUserResponse, PostGetUserResponse, PostGetUsersResponse, PostUpdateUserResponse,
        UpdateUserResponse, UserManagement,
    },
    models, types,
};

use super::{Server, ServerError};

impl UserManagement<Report<ServerError>> for Server {
    async fn change_password(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::ChangePasswordQueryParams,
    ) -> Result<ChangePasswordResponse, ServerError> {
        todo!()
    }

    async fn create_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateUserQueryParams,
    ) -> Result<CreateUserResponse, ServerError> {
        todo!()
    }

    async fn delete_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteUserQueryParams,
    ) -> Result<DeleteUserResponse, ServerError> {
        todo!()
    }

    async fn get_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetUserQueryParams,
    ) -> Result<GetUserResponse, ServerError> {
        todo!()
    }

    async fn get_users(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetUsersResponse, ServerError> {
        todo!()
    }

    async fn post_change_password(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostChangePasswordRequest,
    ) -> Result<PostChangePasswordResponse, ServerError> {
        todo!()
    }

    async fn post_create_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateUserRequest,
    ) -> Result<PostCreateUserResponse, ServerError> {
        todo!()
    }

    async fn post_delete_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteUserRequest,
    ) -> Result<PostDeleteUserResponse, ServerError> {
        todo!()
    }

    async fn post_get_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetUserRequest,
    ) -> Result<PostGetUserResponse, ServerError> {
        todo!()
    }

    async fn post_get_users(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetUsersResponse, ServerError> {
        todo!()
    }

    async fn post_update_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateUserRequest,
    ) -> Result<PostUpdateUserResponse, ServerError> {
        todo!()
    }

    async fn update_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateUserQueryParams,
    ) -> Result<UpdateUserResponse, ServerError> {
        todo!()
    }
}
