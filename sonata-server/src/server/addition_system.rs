#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::addition_system::{
        AdditionSystem, GetOpenSubsonicExtensionsResponse, PostGetOpenSubsonicExtensionsResponse,
    },
    types,
};

use super::{Server, ServerError};

impl AdditionSystem<Report<ServerError>> for Server {
    async fn get_open_subsonic_extensions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetOpenSubsonicExtensionsResponse, ServerError> {
        todo!()
    }

    async fn post_get_open_subsonic_extensions(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetOpenSubsonicExtensionsResponse, ServerError> {
        todo!()
    }
}
