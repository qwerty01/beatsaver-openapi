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
pub struct UserDetail {
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "curator", skip_serializing_if = "Option::is_none")]
    pub curator: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "followData", skip_serializing_if = "Option::is_none")]
    pub follow_data: Option<Box<crate::models::UserFollowData>>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playlistUrl", skip_serializing_if = "Option::is_none")]
    pub playlist_url: Option<String>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<crate::models::UserStats>>,
    #[serde(rename = "suspendedAt", skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<String>,
    #[serde(rename = "testplay", skip_serializing_if = "Option::is_none")]
    pub testplay: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "uniqueSet", skip_serializing_if = "Option::is_none")]
    pub unique_set: Option<bool>,
    #[serde(rename = "uploadLimit", skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i32>,
    #[serde(rename = "verifiedMapper", skip_serializing_if = "Option::is_none")]
    pub verified_mapper: Option<bool>,
}

impl UserDetail {
    pub fn new() -> UserDetail {
        UserDetail {
            admin: None,
            avatar: None,
            curator: None,
            description: None,
            email: None,
            follow_data: None,
            hash: None,
            id: None,
            name: None,
            playlist_url: None,
            stats: None,
            suspended_at: None,
            testplay: None,
            r#type: None,
            unique_set: None,
            upload_limit: None,
            verified_mapper: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "DISCORD")]
    Discord,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "DUAL")]
    Dual,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Discord
    }
}
