use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetInternetRadioStationsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetInternetRadioStationsResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostCreateInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostDeleteInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGetInternetRadioStationsResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::GetInternetRadioStationsResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostUpdateInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
    /// HTTP form POST (`formPost`) Extension not supported
    Status405_HTTPFormPOST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateInternetRadioStationResponse {
    /// Successful or failed response
    Status200_SuccessfulOrFailedResponse(models::SubsonicResponse),
}

/// InternetRadio
#[auto_async_send_sync::auto_send_sync]
#[allow(clippy::ptr_arg)]
pub trait InternetRadio<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// Adds a new internet radio station..
    ///
    /// CreateInternetRadioStation - GET /rest/createInternetRadioStation
    async fn create_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateInternetRadioStationQueryParams,
    ) -> Result<CreateInternetRadioStationResponse, E>;

    /// Deletes an existing internet radio station..
    ///
    /// DeleteInternetRadioStation - GET /rest/deleteInternetRadioStation
    async fn delete_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteInternetRadioStationQueryParams,
    ) -> Result<DeleteInternetRadioStationResponse, E>;

    /// Returns all internet radio stations..
    ///
    /// GetInternetRadioStations - GET /rest/getInternetRadioStations
    async fn get_internet_radio_stations(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetInternetRadioStationsResponse, E>;

    /// Adds a new internet radio station..
    ///
    /// PostCreateInternetRadioStation - POST /rest/createInternetRadioStation
    async fn post_create_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateInternetRadioStationRequest,
    ) -> Result<PostCreateInternetRadioStationResponse, E>;

    /// Deletes an existing internet radio station..
    ///
    /// PostDeleteInternetRadioStation - POST /rest/deleteInternetRadioStation
    async fn post_delete_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteInternetRadioStationRequest,
    ) -> Result<PostDeleteInternetRadioStationResponse, E>;

    /// Returns all internet radio stations..
    ///
    /// PostGetInternetRadioStations - POST /rest/getInternetRadioStations
    async fn post_get_internet_radio_stations(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<crate::types::Object>,
    ) -> Result<PostGetInternetRadioStationsResponse, E>;

    /// Updates an existing internet radio station..
    ///
    /// PostUpdateInternetRadioStation - POST /rest/updateInternetRadioStation
    async fn post_update_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateInternetRadioStationRequest,
    ) -> Result<PostUpdateInternetRadioStationResponse, E>;

    /// Updates an existing internet radio station..
    ///
    /// UpdateInternetRadioStation - GET /rest/updateInternetRadioStation
    async fn update_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateInternetRadioStationQueryParams,
    ) -> Result<UpdateInternetRadioStationResponse, E>;
}
