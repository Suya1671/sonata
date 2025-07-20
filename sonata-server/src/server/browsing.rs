#![allow(unused_variables)]
use axum_extra::extract::{CookieJar, Host};
use error_stack::{Report, Result};
use http::Method;
use sonata_openapi::{
    apis::browsing::{
        Browsing, GetAlbumInfo2Response, GetAlbumInfoResponse, GetAlbumResponse,
        GetArtistInfo2Response, GetArtistInfoResponse, GetArtistResponse, GetArtistsResponse,
        GetGenresResponse, GetIndexesResponse, GetMusicDirectoryResponse, GetMusicFoldersResponse,
        GetSimilarSongs2Response, GetSimilarSongsResponse, GetSongResponse, GetTopSongsResponse,
        GetVideoInfoResponse, GetVideosResponse, PostGetAlbumInfo2Response,
        PostGetAlbumInfoResponse, PostGetAlbumResponse, PostGetArtistInfo2Response,
        PostGetArtistInfoResponse, PostGetArtistResponse, PostGetArtistsResponse,
        PostGetGenresResponse, PostGetIndexesResponse, PostGetMusicDirectoryResponse,
        PostGetMusicFoldersResponse, PostGetSimilarSongs2Response, PostGetSimilarSongsResponse,
        PostGetSongResponse, PostGetTopSongsResponse, PostGetVideoInfoResponse,
        PostGetVideosResponse,
    },
    models, types,
};

use super::{Server, ServerError};

impl Browsing<Report<ServerError>> for Server {
    async fn get_album(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumQueryParams,
    ) -> Result<GetAlbumResponse, ServerError> {
        todo!()
    }

    async fn get_album_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumInfoQueryParams,
    ) -> Result<GetAlbumInfoResponse, ServerError> {
        todo!()
    }

    async fn get_album_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetAlbumInfo2QueryParams,
    ) -> Result<GetAlbumInfo2Response, ServerError> {
        todo!()
    }

    async fn get_artist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistQueryParams,
    ) -> Result<GetArtistResponse, ServerError> {
        todo!()
    }

    async fn get_artist_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistInfoQueryParams,
    ) -> Result<GetArtistInfoResponse, ServerError> {
        todo!()
    }

    async fn get_artist_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistInfo2QueryParams,
    ) -> Result<GetArtistInfo2Response, ServerError> {
        todo!()
    }

    async fn get_artists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetArtistsQueryParams,
    ) -> Result<GetArtistsResponse, ServerError> {
        todo!()
    }

    async fn get_genres(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetGenresResponse, ServerError> {
        todo!()
    }

    async fn get_indexes(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetIndexesQueryParams,
    ) -> Result<GetIndexesResponse, ServerError> {
        todo!()
    }

    async fn get_music_directory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetMusicDirectoryQueryParams,
    ) -> Result<GetMusicDirectoryResponse, ServerError> {
        todo!()
    }

    async fn get_music_folders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetMusicFoldersResponse, ServerError> {
        todo!()
    }

    async fn get_similar_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSimilarSongsQueryParams,
    ) -> Result<GetSimilarSongsResponse, ServerError> {
        todo!()
    }

    async fn get_similar_songs2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSimilarSongs2QueryParams,
    ) -> Result<GetSimilarSongs2Response, ServerError> {
        todo!()
    }

    async fn get_song(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetSongQueryParams,
    ) -> Result<GetSongResponse, ServerError> {
        todo!()
    }

    async fn get_top_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetTopSongsQueryParams,
    ) -> Result<GetTopSongsResponse, ServerError> {
        todo!()
    }

    async fn get_video_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::GetVideoInfoQueryParams,
    ) -> Result<GetVideoInfoResponse, ServerError> {
        todo!()
    }

    async fn get_videos(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetVideosResponse, ServerError> {
        todo!()
    }

    async fn post_get_album(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumRequest,
    ) -> Result<PostGetAlbumResponse, ServerError> {
        todo!()
    }

    async fn post_get_album_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumInfoRequest,
    ) -> Result<PostGetAlbumInfoResponse, ServerError> {
        todo!()
    }

    async fn post_get_album_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetAlbumInfoRequest,
    ) -> Result<PostGetAlbumInfo2Response, ServerError> {
        todo!()
    }

    async fn post_get_artist(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistRequest,
    ) -> Result<PostGetArtistResponse, ServerError> {
        todo!()
    }

    async fn post_get_artist_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistInfoRequest,
    ) -> Result<PostGetArtistInfoResponse, ServerError> {
        todo!()
    }

    async fn post_get_artist_info2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetArtistInfoRequest,
    ) -> Result<PostGetArtistInfo2Response, ServerError> {
        todo!()
    }

    async fn post_get_artists(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetArtistsRequest>,
    ) -> Result<PostGetArtistsResponse, ServerError> {
        todo!()
    }

    async fn post_get_genres(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetGenresResponse, ServerError> {
        todo!()
    }

    async fn post_get_indexes(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::PostGetIndexesRequest>,
    ) -> Result<PostGetIndexesResponse, ServerError> {
        todo!()
    }

    async fn post_get_music_directory(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetMusicDirectoryRequest,
    ) -> Result<PostGetMusicDirectoryResponse, ServerError> {
        todo!()
    }

    async fn post_get_music_folders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetMusicFoldersResponse, ServerError> {
        todo!()
    }

    async fn post_get_similar_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSimilarSongsRequest,
    ) -> Result<PostGetSimilarSongsResponse, ServerError> {
        todo!()
    }

    async fn post_get_similar_songs2(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSimilarSongsRequest,
    ) -> Result<PostGetSimilarSongs2Response, ServerError> {
        todo!()
    }

    async fn post_get_song(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetSongRequest,
    ) -> Result<PostGetSongResponse, ServerError> {
        todo!()
    }

    async fn post_get_top_songs(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetTopSongsRequest,
    ) -> Result<PostGetTopSongsResponse, ServerError> {
        todo!()
    }

    async fn post_get_video_info(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::PostGetVideoInfoRequest,
    ) -> Result<PostGetVideoInfoResponse, ServerError> {
        todo!()
    }

    async fn post_get_videos(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<types::Object>,
    ) -> Result<PostGetVideosResponse, ServerError> {
        todo!()
    }
}
