use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[serde(default)] // Allows deserializing from JSON without an `id` for creation
    pub id: i32,
    pub content: String,
    pub created_by: i32,
    pub has_discussion: bool,
    // The discussion_id is now managed in the PostDiscussion join table.
    // We can add it back here as an Option if we want to join and return it.
    #[sqlx(default)] // This field will be populated by a JOIN query
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostDiscussion {
    #[serde(default)] // Allows deserializing from JSON without an `id` for creation
    pub id: i32,
    pub post_id: i32,
    pub discussion_id: i32,
}

#[derive(Deserialize)]
pub struct CreatePostPayload {
    pub content: String,
    pub created_by: i32,
    pub has_discussion: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdatePostPayload {
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct PostInteraction {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
}