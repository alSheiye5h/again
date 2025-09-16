use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AnnouncementStruct {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub club_id: Option<i32>,
    pub community_id: Option<i32>,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Announcement_create_payload {
    pub title: String,
    pub content: String,
    pub club_id: Option<i32>,
    pub community_id: Option<i32>,
    pub created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct Announcement_update_payload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub club_id: Option<i32>,
    pub community_id: Option<i32>,
}