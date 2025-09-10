use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementStruct {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub target: String,
    pub created_by: String,
    pub created_at: String,
    pub updateda_t: String,
}