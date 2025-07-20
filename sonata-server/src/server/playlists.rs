#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::playlists::{
        CreatePlaylistResponse, DeletePlaylistResponse, GetPlaylistResponse, GetPlaylistsResponse,
        Playlists, PostCreatePlaylistResponse, PostDeletePlaylistResponse, PostGetPlaylistResponse,
        PostGetPlaylistsResponse, PostUpdatePlaylistResponse, UpdatePlaylistResponse,
    },
    models,
};

use super::{Server, ServerError};

impl Playlists<Report<ServerError>> for Server {
    async fn create_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreatePlaylistQueryParams,
    ) -> Result<CreatePlaylistResponse, ServerError> {
        todo!()
    }

    async fn delete_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePlaylistQueryParams,
    ) -> Result<DeletePlaylistResponse, ServerError> {
        todo!()
    }

    async fn get_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPlaylistQueryParams,
    ) -> Result<GetPlaylistResponse, ServerError> {
        todo!()
    }

    async fn get_playlists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPlaylistsQueryParams,
    ) -> Result<GetPlaylistsResponse, ServerError> {
        todo!()
    }

    async fn post_create_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreatePlaylistRequest,
    ) -> Result<PostCreatePlaylistResponse, ServerError> {
        todo!()
    }

    async fn post_delete_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePlaylistRequest,
    ) -> Result<PostDeletePlaylistResponse, ServerError> {
        todo!()
    }

    async fn post_get_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetPlaylistRequest,
    ) -> Result<PostGetPlaylistResponse, ServerError> {
        todo!()
    }

    async fn post_get_playlists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetPlaylistsRequest>,
    ) -> Result<PostGetPlaylistsResponse, ServerError> {
        todo!()
    }

    async fn post_update_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdatePlaylistRequest,
    ) -> Result<PostUpdatePlaylistResponse, ServerError> {
        todo!()
    }

    async fn update_playlist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdatePlaylistQueryParams,
    ) -> Result<UpdatePlaylistResponse, ServerError> {
        todo!()
    }
}
