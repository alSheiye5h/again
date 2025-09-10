use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Post {
    pub id: i32,
    pub created_by: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct PostInteraction {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
}