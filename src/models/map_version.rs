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
pub struct MapVersion {
    #[serde(rename = "coverURL", skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "diffs", skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<models::MapDifficulty>>,
    #[serde(rename = "downloadURL", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "feedback", skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "previewURL", skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(rename = "sageScore", skip_serializing_if = "Option::is_none")]
    pub sage_score: Option<serde_json::Value>,
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "testplayAt", skip_serializing_if = "Option::is_none")]
    pub testplay_at: Option<String>,
    #[serde(rename = "testplays", skip_serializing_if = "Option::is_none")]
    pub testplays: Option<Vec<models::MapTestplay>>,
}

impl MapVersion {
    pub fn new() -> MapVersion {
        MapVersion {
            cover_url: None,
            created_at: None,
            diffs: None,
            download_url: None,
            feedback: None,
            hash: None,
            key: None,
            preview_url: None,
            sage_score: None,
            scheduled_at: None,
            state: None,
            testplay_at: None,
            testplays: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Uploaded")]
    Uploaded,
    #[serde(rename = "Testplay")]
    Testplay,
    #[serde(rename = "Published")]
    Published,
    #[serde(rename = "Feedback")]
    Feedback,
    #[serde(rename = "Scheduled")]
    Scheduled,
}

impl Default for State {
    fn default() -> State {
        Self::Uploaded
    }
}
