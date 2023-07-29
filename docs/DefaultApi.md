# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**maps_hash_hash_get**](DefaultApi.md#maps_hash_hash_get) | **GET** /maps/hash/{hash} | Get map(s) for a map hash
[**maps_id_id_get**](DefaultApi.md#maps_id_id_get) | **GET** /maps/id/{id} | Get map information
[**maps_latest_get**](DefaultApi.md#maps_latest_get) | **GET** /maps/latest | Get maps ordered by upload/publish/updated. If you're going to scrape the data and make 100s of requests make this this endpoint you use.
[**maps_plays_page_get**](DefaultApi.md#maps_plays_page_get) | **GET** /maps/plays/{page} | Get maps ordered by play count (Not currently tracked)
[**maps_uploader_id_page_get**](DefaultApi.md#maps_uploader_id_page_get) | **GET** /maps/uploader/{id}/{page} | Get maps by a user
[**playlists_id_id_batch_post**](DefaultApi.md#playlists_id_id_batch_post) | **POST** /playlists/id/{id}/batch | Add or remove up to 100 maps to a playlist. Requires OAUTH
[**playlists_id_id_page_get**](DefaultApi.md#playlists_id_id_page_get) | **GET** /playlists/id/{id}/{page} | Get playlist detail
[**playlists_latest_get**](DefaultApi.md#playlists_latest_get) | **GET** /playlists/latest | Get playlists ordered by created/updated
[**playlists_search_page_get**](DefaultApi.md#playlists_search_page_get) | **GET** /playlists/search/{page} | Search for playlists
[**playlists_user_user_id_page_get**](DefaultApi.md#playlists_user_user_id_page_get) | **GET** /playlists/user/{userId}/{page} | Get playlists by user
[**search_text_page_get**](DefaultApi.md#search_text_page_get) | **GET** /search/text/{page} | Search for maps
[**users_id_id_get**](DefaultApi.md#users_id_id_get) | **GET** /users/id/{id} | Get user info
[**users_name_name_get**](DefaultApi.md#users_name_name_get) | **GET** /users/name/{name} | Get user info by name
[**users_verify_post**](DefaultApi.md#users_verify_post) | **POST** /users/verify | Verify user tokens
[**vote_get**](DefaultApi.md#vote_get) | **GET** /vote | Get votes
[**vote_post**](DefaultApi.md#vote_post) | **POST** /vote | Vote on a map



## maps_hash_hash_get

> crate::models::MapDetail maps_hash_hash_get(hash)
Get map(s) for a map hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | Up to 50 hashes seperated by commas | [required] |

### Return type

[**crate::models::MapDetail**](MapDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_id_id_get

> crate::models::MapDetail maps_id_id_get(id)
Get map information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id | [required] |

### Return type

[**crate::models::MapDetail**](MapDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_latest_get

> crate::models::SearchResponse maps_latest_get(after, automapper, before, sort)
Get maps ordered by upload/publish/updated. If you're going to scrape the data and make 100s of requests make this this endpoint you use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Like `before` but will get you maps more recent than the time supplied. YYYY-MM-DDTHH:MM:SS+00:00 |  |
**automapper** | Option<**bool**> | true = both, false = no ai |  |
**before** | Option<**String**> | You probably want this. Supplying the uploaded time of the last map in the previous page will get you another page. YYYY-MM-DDTHH:MM:SS+00:00 |  |
**sort** | Option<**String**> | sort |  |

### Return type

[**crate::models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_plays_page_get

> crate::models::SearchResponse maps_plays_page_get(page)
Get maps ordered by play count (Not currently tracked)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i64** | page | [required] |[default to 0]

### Return type

[**crate::models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maps_uploader_id_page_get

> crate::models::SearchResponse maps_uploader_id_page_get(id, page)
Get maps by a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id | [required] |
**page** | **i64** | page | [required] |[default to 0]

### Return type

[**crate::models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_id_batch_post

> crate::models::ActionResponse playlists_id_id_batch_post(id, no_reflection_body)
Add or remove up to 100 maps to a playlist. Requires OAUTH

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id | [required] |
**no_reflection_body** | [**PlaylistBatchRequest**](PlaylistBatchRequest.md) | PlaylistBatchRequest | [required] |

### Return type

[**crate::models::ActionResponse**](ActionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_id_page_get

> crate::models::PlaylistPage playlists_id_id_page_get(id, page)
Get playlist detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id | [required] |
**page** | **i64** | page | [required] |[default to 0]

### Return type

[**crate::models::PlaylistPage**](PlaylistPage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_latest_get

> crate::models::PlaylistSearchResponse playlists_latest_get(after, before, sort)
Get playlists ordered by created/updated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | Like `before` but will get you maps more recent than the time supplied. YYYY-MM-DDTHH:MM:SS+00:00 |  |
**before** | Option<**String**> | You probably want this. Supplying the uploaded time of the last map in the previous page will get you another page. YYYY-MM-DDTHH:MM:SS+00:00 |  |
**sort** | Option<**String**> | sort |  |

### Return type

[**crate::models::PlaylistSearchResponse**](PlaylistSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_search_page_get

> crate::models::PlaylistSearchResponse playlists_search_page_get(page, sort_order, curated, from, include_empty, max_nps, min_nps, q, to, verified)
Search for playlists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i64** | page | [required] |[default to 0]
**sort_order** | **String** | sortOrder | [required] |
**curated** | Option<**bool**> | curated |  |
**from** | Option<**String**> | from |  |
**include_empty** | Option<**bool**> | includeEmpty |  |
**max_nps** | Option<[**serde_json::Value**](.md)> | Float |  |
**min_nps** | Option<[**serde_json::Value**](.md)> | Float |  |
**q** | Option<**String**> | q |  |
**to** | Option<**String**> | to |  |
**verified** | Option<**bool**> | verified |  |

### Return type

[**crate::models::PlaylistSearchResponse**](PlaylistSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_user_user_id_page_get

> crate::models::PlaylistSearchResponse playlists_user_user_id_page_get(page, user_id)
Get playlists by user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i64** | page | [required] |
**user_id** | **i32** | userId | [required] |

### Return type

[**crate::models::PlaylistSearchResponse**](PlaylistSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_text_page_get

> crate::models::SearchResponse search_text_page_get(page, sort_order, automapper, chroma, cinema, curated, from, full_spread, max_bpm, max_duration, max_nps, max_rating, me, min_bpm, min_duration, min_nps, min_rating, noodle, q, ranked, tags, to, verified)
Search for maps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i64** | page | [required] |[default to 0]
**sort_order** | **String** | sortOrder | [required] |
**automapper** | Option<**bool**> | Options are a little weird, I may add another enum field in future to make this clearer. true = both, false = only ai, null = no ai |  |
**chroma** | Option<**bool**> | chroma |  |
**cinema** | Option<**bool**> | cinema |  |
**curated** | Option<**bool**> | curated |  |
**from** | Option<**String**> | from |  |
**full_spread** | Option<**bool**> | fullSpread |  |
**max_bpm** | Option<[**serde_json::Value**](.md)> | Float |  |
**max_duration** | Option<**i32**> | maxDuration |  |
**max_nps** | Option<[**serde_json::Value**](.md)> | Float |  |
**max_rating** | Option<[**serde_json::Value**](.md)> | Float |  |
**me** | Option<**bool**> | me |  |
**min_bpm** | Option<[**serde_json::Value**](.md)> | Float |  |
**min_duration** | Option<**i32**> | minDuration |  |
**min_nps** | Option<[**serde_json::Value**](.md)> | Float |  |
**min_rating** | Option<[**serde_json::Value**](.md)> | Float |  |
**noodle** | Option<**bool**> | noodle |  |
**q** | Option<**String**> | q |  |
**ranked** | Option<**bool**> | ranked |  |
**tags** | Option<**String**> | Tag query, separated by `,` (and) or `|` (or). Excluded tags are prefixed with `!`. |  |
**to** | Option<**String**> | to |  |
**verified** | Option<**bool**> | verified |  |

### Return type

[**crate::models::SearchResponse**](SearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_id_id_get

> crate::models::UserDetail users_id_id_get(id)
Get user info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | id | [required] |

### Return type

[**crate::models::UserDetail**](UserDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_name_name_get

> crate::models::UserDetail users_name_name_get(name)
Get user info by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name | [required] |

### Return type

[**crate::models::UserDetail**](UserDetail.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_verify_post

> crate::models::VerifyResponse users_verify_post(no_reflection_body)
Verify user tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**no_reflection_body** | [**AuthRequest**](AuthRequest.md) | AuthRequest | [required] |

### Return type

[**crate::models::VerifyResponse**](VerifyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_get

> Vec<crate::models::VoteSummary> vote_get(since)
Get votes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | **String** | since | [required] |

### Return type

[**Vec<crate::models::VoteSummary>**](VoteSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_post

> crate::models::VoteResponse vote_post(no_reflection_body)
Vote on a map

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**no_reflection_body** | [**VoteRequest**](VoteRequest.md) | VoteRequest | [required] |

### Return type

[**crate::models::VoteResponse**](VoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

