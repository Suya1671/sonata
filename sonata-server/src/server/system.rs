use super::{Server, ServerError};
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    API_VERSION,
    apis::system::{GetLicenseResponse, PingResponse, PostGetLicenseResponse, PostPingResponse},
    models, types,
};

impl sonata_openapi::apis::system::System<Report<ServerError>> for Server {
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
        Ok(PingResponse::Status200_SuccessfulOrFailedResponse(
            models::SubsonicResponse {
                subsonic_response: Some(
                    models::SubsonicSuccessResponse {
                        version: API_VERSION.to_string(),
                        r#type: "Sonata".to_string(),
                        server_version: env!("CARGO_PKG_VERSION").to_string(),
                        open_subsonic: true,
                        status: "ok".to_string(),
                    }
                    .into(),
                ),
            },
        ))
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
        Ok(match self.ping(method, host, cookies).await? {
            PingResponse::Status200_SuccessfulOrFailedResponse(res) => {
                PostPingResponse::Status200_SuccessfulOrFailedResponse(res)
            }
        })
    }
}
