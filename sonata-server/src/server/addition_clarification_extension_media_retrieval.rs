use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::addition_clarification_extension_media_retrieval::{
        AdditionClarificationExtensionMediaRetrieval, PostStreamResponse, StreamResponse
    },
    models, types,
};

use super::{Server, ServerError};

impl AdditionClarificationExtensionMediaRetrieval<Report<ServerError>> for Server {
    async fn post_stream(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostStreamRequest
    ) -> Result<PostStreamResponse, ServerError> {
        todo!()
    }

    async fn stream(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::StreamQueryParams
    ) -> Result<StreamResponse, ServerError> {
        todo!()
    }

}
