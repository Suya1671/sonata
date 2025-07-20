use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::addition_extension_system::{
        AdditionExtensionSystem, PostTokenInfoResponse, TokenInfoResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl AdditionExtensionSystem<Report<ServerError>> for Server {
    async fn post_token_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostTokenInfoResponse, ServerError> {
        todo!()
    }

    async fn token_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<TokenInfoResponse, ServerError> {
        todo!()
    }
}
