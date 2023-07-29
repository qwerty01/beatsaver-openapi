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
pub struct MapStats {
    #[serde(rename = "downloads", skip_serializing_if = "Option::is_none")]
    pub downloads: Option<i32>,
    #[serde(rename = "downvotes", skip_serializing_if = "Option::is_none")]
    pub downvotes: Option<i32>,
    #[serde(rename = "plays", skip_serializing_if = "Option::is_none")]
    pub plays: Option<i32>,
    #[serde(rename = "reviews", skip_serializing_if = "Option::is_none")]
    pub reviews: Option<i32>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<serde_json::Value>,
    #[serde(rename = "scoreOneDP", skip_serializing_if = "Option::is_none")]
    pub score_one_dp: Option<serde_json::Value>,
    #[serde(rename = "sentiment", skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<Sentiment>,
    #[serde(rename = "upvotes", skip_serializing_if = "Option::is_none")]
    pub upvotes: Option<i32>,
}

impl MapStats {
    pub fn new() -> MapStats {
        MapStats {
            downloads: None,
            downvotes: None,
            plays: None,
            reviews: None,
            score: None,
            score_one_dp: None,
            sentiment: None,
            upvotes: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sentiment {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "VERY_NEGATIVE")]
    VeryNegative,
    #[serde(rename = "MOSTLY_NEGATIVE")]
    MostlyNegative,
    #[serde(rename = "MIXED")]
    Mixed,
    #[serde(rename = "MOSTLY_POSITIVE")]
    MostlyPositive,
    #[serde(rename = "VERY_POSITIVE")]
    VeryPositive,
}

impl Default for Sentiment {
    fn default() -> Sentiment {
        Self::Pending
    }
}

