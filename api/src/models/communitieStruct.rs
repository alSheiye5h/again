use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use serde_json::Value as JsonValue;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Community {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub bio: String,
    pub created_at: NaiveDateTime,
    pub privacy_state: bool,
    pub rules: Option<JsonValue>,
}

/// Payload for creating a new community.
#[derive(Deserialize)]
pub struct CreateCommunityPayload {
    pub created_by: i32,
    pub name: String,
    pub bio: String,
    pub privacy_state: bool,
    pub rules: Option<JsonValue>,
}

/// Payload for updating an existing community.
#[derive(Deserialize)]
pub struct UpdateCommunityPayload {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub privacy_state: Option<bool>,
    pub rules: Option<JsonValue>,
}
