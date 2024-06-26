/*
 * BeatSaver API
 *
 * WIP  If you want to keep any kind of mirror instead of making 100s of requests instead consider subscribing to the websocket api. wss://ws.beatsaver.com/maps  Messages will be in the style {\"type\": \"MAP_UPDATE\", \"msg\": __MAP_DATA_HERE__}
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistFull {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "curatedAt", skip_serializing_if = "Option::is_none")]
    pub curated_at: Option<String>,
    #[serde(rename = "curator", skip_serializing_if = "Option::is_none")]
    pub curator: Option<Box<models::UserDetail>>,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "downloadURL", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::UserDetail>>,
    #[serde(rename = "playlistId", skip_serializing_if = "Option::is_none")]
    pub playlist_id: Option<i32>,
    #[serde(rename = "playlistImage", skip_serializing_if = "Option::is_none")]
    pub playlist_image: Option<String>,
    #[serde(rename = "playlistImage512", skip_serializing_if = "Option::is_none")]
    pub playlist_image512: Option<String>,
    #[serde(rename = "songsChangedAt", skip_serializing_if = "Option::is_none")]
    pub songs_changed_at: Option<String>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<models::PlaylistStats>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl PlaylistFull {
    pub fn new() -> PlaylistFull {
        PlaylistFull {
            config: None,
            created_at: None,
            curated_at: None,
            curator: None,
            deleted_at: None,
            description: None,
            download_url: None,
            name: None,
            owner: None,
            playlist_id: None,
            playlist_image: None,
            playlist_image512: None,
            songs_changed_at: None,
            stats: None,
            r#type: None,
            updated_at: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Private")]
    Private,
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "System")]
    System,
    #[serde(rename = "Search")]
    Search,
}

impl Default for Type {
    fn default() -> Type {
        Self::Private
    }
}
