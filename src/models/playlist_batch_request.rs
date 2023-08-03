/*
 * BeatSaver API
 *
 * WIP  If you want to keep any kind of mirror instead of making 100s of requests instead consider subscribing to the websocket api. wss://ws.beatsaver.com/maps  Messages will be in the style {\"type\": \"MAP_UPDATE\", \"msg\": __MAP_DATA_HERE__}
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlaylistBatchRequest {
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<String>>,
    #[serde(rename = "ignoreUnknown", skip_serializing_if = "Option::is_none")]
    pub ignore_unknown: Option<bool>,
    #[serde(rename = "inPlaylist", skip_serializing_if = "Option::is_none")]
    pub in_playlist: Option<bool>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

impl PlaylistBatchRequest {
    pub fn new() -> PlaylistBatchRequest {
        PlaylistBatchRequest {
            hashes: None,
            ignore_unknown: None,
            in_playlist: None,
            keys: None,
        }
    }
}
