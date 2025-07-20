use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::media_retrieval::{
        DownloadResponse, GetAvatarResponse, GetCaptionsResponse, GetCoverArtResponse,
        GetLyricsResponse, HlsPeriodM3u8Response, MediaRetrieval, PostDownloadResponse,
        PostGetAvatarResponse, PostGetCaptionsResponse, PostGetCoverArtResponse,
        PostGetLyricsResponse, PostHlsPeriodM3u8Response,
    },
    models,
};

use super::{Server, ServerError};

impl MediaRetrieval<Report<ServerError>> for Server {
    async fn download(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DownloadQueryParams,
    ) -> Result<DownloadResponse, ServerError> {
        todo!()
    }

    async fn get_avatar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAvatarQueryParams,
    ) -> Result<GetAvatarResponse, ServerError> {
        todo!()
    }

    async fn get_captions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetCaptionsQueryParams,
    ) -> Result<GetCaptionsResponse, ServerError> {
        todo!()
    }

    async fn get_cover_art(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetCoverArtQueryParams,
    ) -> Result<GetCoverArtResponse, ServerError> {
        todo!()
    }

    async fn get_lyrics(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetLyricsQueryParams,
    ) -> Result<GetLyricsResponse, ServerError> {
        todo!()
    }

    async fn hls_period_m3u8(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::HlsPeriodM3u8QueryParams,
    ) -> Result<HlsPeriodM3u8Response, ServerError> {
        todo!()
    }

    async fn post_download(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDownloadRequest,
    ) -> Result<PostDownloadResponse, ServerError> {
        todo!()
    }

    async fn post_get_avatar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAvatarRequest,
    ) -> Result<PostGetAvatarResponse, ServerError> {
        todo!()
    }

    async fn post_get_captions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetCaptionsRequest,
    ) -> Result<PostGetCaptionsResponse, ServerError> {
        todo!()
    }

    async fn post_get_cover_art(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetCoverArtRequest,
    ) -> Result<PostGetCoverArtResponse, ServerError> {
        todo!()
    }

    async fn post_get_lyrics(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetLyricsRequest>,
    ) -> Result<PostGetLyricsResponse, ServerError> {
        todo!()
    }

    async fn post_hls_period_m3u8(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostHlsM3u8Request,
    ) -> Result<PostHlsPeriodM3u8Response, ServerError> {
        todo!()
    }
}
