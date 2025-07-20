use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::system::{
        GetLicenseResponse, PingResponse, PostGetLicenseResponse, PostPingResponse, System,
    },
    types,
};

use super::{Server, ServerError};

impl System<Report<ServerError>> for Server {
    async fn get_license(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetLicenseResponse, ServerError> {
        todo!()
    }

    async fn ping(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<PingResponse, ServerError> {
        todo!()
    }

    async fn post_get_license(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetLicenseResponse, ServerError> {
        todo!()
    }

    async fn post_ping(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostPingResponse, ServerError> {
        todo!()
    }
}
