#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::podcast::{
        CreatePodcastChannelResponse, DeletePodcastChannelResponse, DeletePodcastEpisodeResponse,
        DownloadPodcastEpisodeResponse, GetNewestPodcastsResponse, GetPodcastsResponse, Podcast,
        PostCreatePodcastChannelResponse, PostDeletePodcastChannelResponse,
        PostDeletePodcastEpisodeResponse, PostDownloadPodcastEpisodeResponse,
        PostGetNewestPodcastsResponse, PostGetPodcastsResponse, PostRefreshPodcastsResponse,
        RefreshPodcastsResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Podcast<Report<ServerError>> for Server {
    async fn create_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreatePodcastChannelQueryParams,
    ) -> Result<CreatePodcastChannelResponse, ServerError> {
        todo!()
    }

    async fn delete_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePodcastChannelQueryParams,
    ) -> Result<DeletePodcastChannelResponse, ServerError> {
        todo!()
    }

    async fn delete_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeletePodcastEpisodeQueryParams,
    ) -> Result<DeletePodcastEpisodeResponse, ServerError> {
        todo!()
    }

    async fn download_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DownloadPodcastEpisodeQueryParams,
    ) -> Result<DownloadPodcastEpisodeResponse, ServerError> {
        todo!()
    }

    async fn get_newest_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetNewestPodcastsQueryParams,
    ) -> Result<GetNewestPodcastsResponse, ServerError> {
        todo!()
    }

    async fn get_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPodcastsQueryParams,
    ) -> Result<GetPodcastsResponse, ServerError> {
        todo!()
    }

    async fn post_create_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreatePodcastChannelRequest,
    ) -> Result<PostCreatePodcastChannelResponse, ServerError> {
        todo!()
    }

    async fn post_delete_podcast_channel(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePodcastChannelRequest,
    ) -> Result<PostDeletePodcastChannelResponse, ServerError> {
        todo!()
    }

    async fn post_delete_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeletePodcastEpisodeRequest,
    ) -> Result<PostDeletePodcastEpisodeResponse, ServerError> {
        todo!()
    }

    async fn post_download_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDownloadPodcastEpisodeRequest,
    ) -> Result<PostDownloadPodcastEpisodeResponse, ServerError> {
        todo!()
    }

    async fn post_get_newest_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetNewestPodcastsRequest>,
    ) -> Result<PostGetNewestPodcastsResponse, ServerError> {
        todo!()
    }

    async fn post_get_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetPodcastsRequest>,
    ) -> Result<PostGetPodcastsResponse, ServerError> {
        todo!()
    }

    async fn post_refresh_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostRefreshPodcastsResponse, ServerError> {
        todo!()
    }

    async fn refresh_podcasts(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<RefreshPodcastsResponse, ServerError> {
        todo!()
    }
}
