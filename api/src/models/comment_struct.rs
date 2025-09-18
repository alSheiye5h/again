use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateCommentPayload {
    pub content: String,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateCommentPayload {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub post_id: i32,
}