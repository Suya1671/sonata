#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::internet_radio::{
        CreateInternetRadioStationResponse, DeleteInternetRadioStationResponse,
        GetInternetRadioStationsResponse, InternetRadio, PostCreateInternetRadioStationResponse,
        PostDeleteInternetRadioStationResponse, PostGetInternetRadioStationsResponse,
        PostUpdateInternetRadioStationResponse, UpdateInternetRadioStationResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl InternetRadio<Report<ServerError>> for Server {
    async fn create_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CreateInternetRadioStationQueryParams,
    ) -> Result<CreateInternetRadioStationResponse, ServerError> {
        todo!()
    }

    async fn delete_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::DeleteInternetRadioStationQueryParams,
    ) -> Result<DeleteInternetRadioStationResponse, ServerError> {
        todo!()
    }

    async fn get_internet_radio_stations(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetInternetRadioStationsResponse, ServerError> {
        todo!()
    }

    async fn post_create_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostCreateInternetRadioStationRequest,
    ) -> Result<PostCreateInternetRadioStationResponse, ServerError> {
        todo!()
    }

    async fn post_delete_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostDeleteInternetRadioStationRequest,
    ) -> Result<PostDeleteInternetRadioStationResponse, ServerError> {
        todo!()
    }

    async fn post_get_internet_radio_stations(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetInternetRadioStationsResponse, ServerError> {
        todo!()
    }

    async fn post_update_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostUpdateInternetRadioStationRequest,
    ) -> Result<PostUpdateInternetRadioStationResponse, ServerError> {
        todo!()
    }

    async fn update_internet_radio_station(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::UpdateInternetRadioStationQueryParams,
    ) -> Result<UpdateInternetRadioStationResponse, ServerError> {
        todo!()
    }
}
