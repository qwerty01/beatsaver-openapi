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
pub struct ActionResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ActionResponse {
    pub fn new() -> ActionResponse {
        ActionResponse {
            errors: None,
            success: None,
        }
    }
}
