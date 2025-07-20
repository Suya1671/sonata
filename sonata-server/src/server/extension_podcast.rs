use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::extension_podcast::{
        ExtensionPodcast, GetPodcastEpisodeResponse, PostGetPodcastEpisodeResponse,
    },
    models,
};

use super::{Server, ServerError};

impl ExtensionPodcast<Report<ServerError>> for Server {
    async fn get_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetPodcastEpisodeQueryParams,
    ) -> Result<GetPodcastEpisodeResponse, ServerError> {
        todo!()
    }

    async fn post_get_podcast_episode(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetPodcastEpisodeRequest,
    ) -> Result<PostGetPodcastEpisodeResponse, ServerError> {
        todo!()
    }
}
