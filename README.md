# Rust API client for openapi

WIP

If you want to keep any kind of mirror instead of making 100s of requests instead consider subscribing to the websocket api.
wss://ws.beatsaver.com/maps

Messages will be in the style {\"type\": \"MAP_UPDATE\", \"msg\": __MAP_DATA_HERE__}


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1
- Package version: 0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**maps_hash_hash_get**](docs/DefaultApi.md#maps_hash_hash_get) | **GET** /maps/hash/{hash} | Get map(s) for a map hash
*DefaultApi* | [**maps_id_id_get**](docs/DefaultApi.md#maps_id_id_get) | **GET** /maps/id/{id} | Get map information
*DefaultApi* | [**maps_latest_get**](docs/DefaultApi.md#maps_latest_get) | **GET** /maps/latest | Get maps ordered by upload/publish/updated. If you're going to scrape the data and make 100s of requests make this this endpoint you use.
*DefaultApi* | [**maps_plays_page_get**](docs/DefaultApi.md#maps_plays_page_get) | **GET** /maps/plays/{page} | Get maps ordered by play count (Not currently tracked)
*DefaultApi* | [**maps_uploader_id_page_get**](docs/DefaultApi.md#maps_uploader_id_page_get) | **GET** /maps/uploader/{id}/{page} | Get maps by a user
*DefaultApi* | [**playlists_id_id_batch_post**](docs/DefaultApi.md#playlists_id_id_batch_post) | **POST** /playlists/id/{id}/batch | Add or remove up to 100 maps to a playlist. Requires OAUTH
*DefaultApi* | [**playlists_id_id_page_get**](docs/DefaultApi.md#playlists_id_id_page_get) | **GET** /playlists/id/{id}/{page} | Get playlist detail
*DefaultApi* | [**playlists_latest_get**](docs/DefaultApi.md#playlists_latest_get) | **GET** /playlists/latest | Get playlists ordered by created/updated
*DefaultApi* | [**playlists_search_page_get**](docs/DefaultApi.md#playlists_search_page_get) | **GET** /playlists/search/{page} | Search for playlists
*DefaultApi* | [**playlists_user_user_id_page_get**](docs/DefaultApi.md#playlists_user_user_id_page_get) | **GET** /playlists/user/{userId}/{page} | Get playlists by user
*DefaultApi* | [**search_text_page_get**](docs/DefaultApi.md#search_text_page_get) | **GET** /search/text/{page} | Search for maps
*DefaultApi* | [**users_id_id_get**](docs/DefaultApi.md#users_id_id_get) | **GET** /users/id/{id} | Get user info
*DefaultApi* | [**users_name_name_get**](docs/DefaultApi.md#users_name_name_get) | **GET** /users/name/{name} | Get user info by name
*DefaultApi* | [**users_verify_post**](docs/DefaultApi.md#users_verify_post) | **POST** /users/verify | Verify user tokens
*DefaultApi* | [**vote_get**](docs/DefaultApi.md#vote_get) | **GET** /vote | Get votes
*DefaultApi* | [**vote_post**](docs/DefaultApi.md#vote_post) | **POST** /vote | Vote on a map


## Documentation For Models

 - [ActionResponse](docs/ActionResponse.md)
 - [AuthRequest](docs/AuthRequest.md)
 - [MapDetail](docs/MapDetail.md)
 - [MapDetailMetadata](docs/MapDetailMetadata.md)
 - [MapDetailWithOrder](docs/MapDetailWithOrder.md)
 - [MapDifficulty](docs/MapDifficulty.md)
 - [MapParitySummary](docs/MapParitySummary.md)
 - [MapStats](docs/MapStats.md)
 - [MapTestplay](docs/MapTestplay.md)
 - [MapVersion](docs/MapVersion.md)
 - [PlaylistBatchRequest](docs/PlaylistBatchRequest.md)
 - [PlaylistFull](docs/PlaylistFull.md)
 - [PlaylistPage](docs/PlaylistPage.md)
 - [PlaylistSearchResponse](docs/PlaylistSearchResponse.md)
 - [PlaylistStats](docs/PlaylistStats.md)
 - [SearchResponse](docs/SearchResponse.md)
 - [UserDetail](docs/UserDetail.md)
 - [UserDiffStats](docs/UserDiffStats.md)
 - [UserFollowData](docs/UserFollowData.md)
 - [UserStats](docs/UserStats.md)
 - [VerifyResponse](docs/VerifyResponse.md)
 - [VoteRequest](docs/VoteRequest.md)
 - [VoteResponse](docs/VoteResponse.md)
 - [VoteSummary](docs/VoteSummary.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



