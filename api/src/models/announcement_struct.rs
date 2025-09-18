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
#[derive(sqlx::FromRow, Serialize)]
pub struct DiscussionAnnouncement {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_by: i32,
    pub discussion_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateAnnouncementPayload {
    pub title: String,
    pub content: String,
    pub created_by: i32,
}


#[derive(Debug, Deserialize)]
pub struct AnnouncementCreatePayload {
    pub title: String,
    pub content: String,
    pub club_id: Option<i32>,
    pub community_id: Option<i32>,
    pub created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct AnnouncementUpdatePayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub club_id: Option<i32>,
    pub community_id: Option<i32>,
}