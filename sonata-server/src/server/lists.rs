use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::lists::{
        GetAlbumList2Response, GetAlbumListResponse, GetNowPlayingResponse, GetRandomSongsResponse,
        GetSongsByGenreResponse, GetStarred2Response, GetStarredResponse, Lists,
        PostGetAlbumList2Response, PostGetAlbumListResponse, PostGetNowPlayingResponse,
        PostGetRandomSongsResponse, PostGetSongsByGenreResponse, PostGetStarred2Response,
        PostGetStarredResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Lists<Report<ServerError>> for Server {
    async fn get_album_list(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumListQueryParams,
    ) -> Result<GetAlbumListResponse, ServerError> {
        todo!()
    }

    async fn get_album_list2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumList2QueryParams,
    ) -> Result<GetAlbumList2Response, ServerError> {
        todo!()
    }

    async fn get_now_playing(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetNowPlayingResponse, ServerError> {
        todo!()
    }

    async fn get_random_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetRandomSongsQueryParams,
    ) -> Result<GetRandomSongsResponse, ServerError> {
        todo!()
    }

    async fn get_songs_by_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSongsByGenreQueryParams,
    ) -> Result<GetSongsByGenreResponse, ServerError> {
        todo!()
    }

    async fn get_starred(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetStarredQueryParams,
    ) -> Result<GetStarredResponse, ServerError> {
        todo!()
    }

    async fn get_starred2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetStarred2QueryParams,
    ) -> Result<GetStarred2Response, ServerError> {
        todo!()
    }

    async fn post_get_album_list(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumListRequest,
    ) -> Result<PostGetAlbumListResponse, ServerError> {
        todo!()
    }

    async fn post_get_album_list2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumListRequest,
    ) -> Result<PostGetAlbumList2Response, ServerError> {
        todo!()
    }

    async fn post_get_now_playing(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetNowPlayingResponse, ServerError> {
        todo!()
    }

    async fn post_get_random_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetRandomSongsRequest>,
    ) -> Result<PostGetRandomSongsResponse, ServerError> {
        todo!()
    }

    async fn post_get_songs_by_genre(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSongsByGenreRequest,
    ) -> Result<PostGetSongsByGenreResponse, ServerError> {
        todo!()
    }

    async fn post_get_starred(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetStarredResponse, ServerError> {
        todo!()
    }

    async fn post_get_starred2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetStarred2Response, ServerError> {
        todo!()
    }
}
