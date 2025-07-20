#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::media_library_scanning::{
        GetScanStatusResponse, MediaLibraryScanning, PostGetScanStatusResponse,
        PostStartScanResponse, StartScanResponse,
    },
    types,
};

use super::{Server, ServerError};

impl MediaLibraryScanning<Report<ServerError>> for Server {
    async fn get_scan_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetScanStatusResponse, ServerError> {
        todo!()
    }

    async fn post_get_scan_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetScanStatusResponse, ServerError> {
        todo!()
    }

    async fn post_start_scan(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostStartScanResponse, ServerError> {
        todo!()
    }

    async fn start_scan(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<StartScanResponse, ServerError> {
        todo!()
    }
}
