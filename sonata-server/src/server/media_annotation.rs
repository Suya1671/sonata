use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::media_annotation::{
        MediaAnnotation, PostScrobbleResponse, PostSetRatingResponse, PostStarResponse,
        PostUnstarResponse, ScrobbleResponse, SetRatingResponse, StarResponse, UnstarResponse,
    },
    models,
};

use super::{Server, ServerError};

impl MediaAnnotation<Report<ServerError>> for Server {
    async fn post_scrobble(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostScrobbleRequest,
    ) -> Result<PostScrobbleResponse, ServerError> {
        todo!()
    }

    async fn post_set_rating(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSetRatingRequest,
    ) -> Result<PostSetRatingResponse, ServerError> {
        todo!()
    }

    async fn post_star(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostStarRequest,
    ) -> Result<PostStarResponse, ServerError> {
        todo!()
    }

    async fn post_unstar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUnstarRequest,
    ) -> Result<PostUnstarResponse, ServerError> {
        todo!()
    }

    async fn scrobble(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::ScrobbleQueryParams,
    ) -> Result<ScrobbleResponse, ServerError> {
        todo!()
    }

    async fn set_rating(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::SetRatingQueryParams,
    ) -> Result<SetRatingResponse, ServerError> {
        todo!()
    }

    async fn star(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::StarQueryParams,
    ) -> Result<StarResponse, ServerError> {
        todo!()
    }

    async fn unstar(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UnstarQueryParams,
    ) -> Result<UnstarResponse, ServerError> {
        todo!()
    }
}
