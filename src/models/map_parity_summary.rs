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
pub struct MapParitySummary {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<i32>,
    #[serde(rename = "resets", skip_serializing_if = "Option::is_none")]
    pub resets: Option<i32>,
    #[serde(rename = "warns", skip_serializing_if = "Option::is_none")]
    pub warns: Option<i32>,
}

impl MapParitySummary {
    pub fn new() -> MapParitySummary {
        MapParitySummary {
            errors: None,
            resets: None,
            warns: None,
        }
    }
}
