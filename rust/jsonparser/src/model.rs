use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Twitter {
    pub statuses: Vec<Status>,
    #[serde(rename = "search_metadata")]
    pub search_metadata: SearchMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub coordinates: Value,
    pub favorited: bool,
    pub truncated: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub entities: Entities,
    #[serde(rename = "in_reply_to_user_id_str")]
    pub in_reply_to_user_id_str: Value,
    pub contributors: Value,
    pub text: String,
    pub metadata: Metadata,
    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,
    #[serde(rename = "in_reply_to_status_id_str")]
    pub in_reply_to_status_id_str: Value,
    pub id: i64,
    pub geo: Value,
    pub retweeted: bool,
    #[serde(rename = "in_reply_to_user_id")]
    pub in_reply_to_user_id: Value,
    pub place: Value,
    pub user: User,
    #[serde(rename = "in_reply_to_screen_name")]
    pub in_reply_to_screen_name: Value,
    pub source: String,
    #[serde(rename = "in_reply_to_status_id")]
    pub in_reply_to_status_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub urls: Vec<Value>,
    pub hashtags: Vec<Hashtag>,
    #[serde(rename = "user_mentions")]
    pub user_mentions: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashtag {
    pub text: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "iso_language_code")]
    pub iso_language_code: String,
    #[serde(rename = "result_type")]
    pub result_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "profile_sidebar_fill_color")]
    pub profile_sidebar_fill_color: String,
    #[serde(rename = "profile_sidebar_border_color")]
    pub profile_sidebar_border_color: String,
    #[serde(rename = "profile_background_tile")]
    pub profile_background_tile: bool,
    pub name: String,
    #[serde(rename = "profile_image_url")]
    pub profile_image_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub location: String,
    #[serde(rename = "follow_request_sent")]
    pub follow_request_sent: Value,
    #[serde(rename = "profile_link_color")]
    pub profile_link_color: String,
    #[serde(rename = "is_translator")]
    pub is_translator: bool,
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub entities: Entities2,
    #[serde(rename = "default_profile")]
    pub default_profile: bool,
    #[serde(rename = "contributors_enabled")]
    pub contributors_enabled: bool,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    pub url: Option<String>,
    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,
    #[serde(rename = "utc_offset")]
    pub utc_offset: i64,
    pub id: i64,
    #[serde(rename = "profile_use_background_image")]
    pub profile_use_background_image: bool,
    #[serde(rename = "listed_count")]
    pub listed_count: i64,
    #[serde(rename = "profile_text_color")]
    pub profile_text_color: String,
    pub lang: String,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    pub protected: bool,
    pub notifications: Value,
    #[serde(rename = "profile_background_image_url_https")]
    pub profile_background_image_url_https: String,
    #[serde(rename = "profile_background_color")]
    pub profile_background_color: String,
    pub verified: bool,
    #[serde(rename = "geo_enabled")]
    pub geo_enabled: bool,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    pub description: String,
    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,
    #[serde(rename = "profile_background_image_url")]
    pub profile_background_image_url: String,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "friends_count")]
    pub friends_count: i64,
    pub following: Value,
    #[serde(rename = "show_all_inline_media")]
    pub show_all_inline_media: bool,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities2 {
    pub url: Url,
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub urls: Vec<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url2 {
    #[serde(rename = "expanded_url")]
    pub expanded_url: Value,
    pub url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMetadata {
    #[serde(rename = "max_id")]
    pub max_id: i64,
    #[serde(rename = "since_id")]
    pub since_id: i64,
    #[serde(rename = "refresh_url")]
    pub refresh_url: String,
    #[serde(rename = "next_results")]
    pub next_results: String,
    pub count: i64,
    #[serde(rename = "completed_in")]
    pub completed_in: f64,
    #[serde(rename = "since_id_str")]
    pub since_id_str: String,
    pub query: String,
    #[serde(rename = "max_id_str")]
    pub max_id_str: String,
}
