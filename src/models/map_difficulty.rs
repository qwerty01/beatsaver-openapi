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
pub struct MapDifficulty {
    #[serde(rename = "bombs", skip_serializing_if = "Option::is_none")]
    pub bombs: Option<i32>,
    #[serde(rename = "characteristic", skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Characteristic>,
    #[serde(rename = "chroma", skip_serializing_if = "Option::is_none")]
    pub chroma: Option<bool>,
    #[serde(rename = "cinema", skip_serializing_if = "Option::is_none")]
    pub cinema: Option<bool>,
    #[serde(rename = "difficulty", skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<Difficulty>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<i32>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(rename = "maxScore", skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i32>,
    #[serde(rename = "me", skip_serializing_if = "Option::is_none")]
    pub me: Option<bool>,
    #[serde(rename = "ne", skip_serializing_if = "Option::is_none")]
    pub ne: Option<bool>,
    #[serde(rename = "njs", skip_serializing_if = "Option::is_none")]
    pub njs: Option<serde_json::Value>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<i32>,
    #[serde(rename = "nps", skip_serializing_if = "Option::is_none")]
    pub nps: Option<f64>,
    #[serde(rename = "obstacles", skip_serializing_if = "Option::is_none")]
    pub obstacles: Option<i32>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<serde_json::Value>,
    #[serde(rename = "paritySummary", skip_serializing_if = "Option::is_none")]
    pub parity_summary: Option<Box<crate::models::MapParitySummary>>,
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
    #[serde(rename = "stars", skip_serializing_if = "Option::is_none")]
    pub stars: Option<serde_json::Value>,
}

impl MapDifficulty {
    pub fn new() -> MapDifficulty {
        MapDifficulty {
            bombs: None,
            characteristic: None,
            chroma: None,
            cinema: None,
            difficulty: None,
            events: None,
            label: None,
            length: None,
            max_score: None,
            me: None,
            ne: None,
            njs: None,
            notes: None,
            nps: None,
            obstacles: None,
            offset: None,
            parity_summary: None,
            seconds: None,
            stars: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Characteristic {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "OneSaber")]
    OneSaber,
    #[serde(rename = "NoArrows")]
    NoArrows,
    #[serde(rename = "90Degree")]
    Variant90Degree,
    #[serde(rename = "360Degree")]
    Variant360Degree,
    #[serde(rename = "Lightshow")]
    Lightshow,
    #[serde(rename = "Lawless")]
    Lawless,
    #[serde(rename = "Legacy")]
    Legacy,
}

impl Default for Characteristic {
    fn default() -> Characteristic {
        Self::Standard
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "Easy")]
    Easy,
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Hard")]
    Hard,
    #[serde(rename = "Expert")]
    Expert,
    #[serde(rename = "ExpertPlus")]
    ExpertPlus,
}

impl Default for Difficulty {
    fn default() -> Difficulty {
        Self::Easy
    }
}
