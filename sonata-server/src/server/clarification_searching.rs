use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::clarification_searching::{ClarificationSearching, PostSearch3Response, Search3Response},
    models,
};

use super::{Server, ServerError};

impl ClarificationSearching<Report<ServerError>> for Server {
    async fn post_search3(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostSearch3Request,
    ) -> Result<PostSearch3Response, ServerError> {
        todo!()
    }

    async fn search3(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::Search3QueryParams,
    ) -> Result<Search3Response, ServerError> {
        todo!()
    }
}
