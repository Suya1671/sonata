#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::jukebox::{Jukebox, JukeboxControlResponse, PostJukeboxControlResponse},
    models,
};

use super::{Server, ServerError};

impl Jukebox<Report<ServerError>> for Server {
    async fn jukebox_control(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::JukeboxControlQueryParams,
    ) -> Result<JukeboxControlResponse, ServerError> {
        todo!()
    }

    async fn post_jukebox_control(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostJukeboxControlRequest,
    ) -> Result<PostJukeboxControlResponse, ServerError> {
        todo!()
    }
}
