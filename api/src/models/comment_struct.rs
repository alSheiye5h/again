use serde::{Deserialize, Serialize};
use sqlx::Type;

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

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum CommentInteractionType {
    Like,
    Upvote,
    Downvote,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommentInteraction {
    pub id: i32,
    pub user_id: i32,
    pub comment_id: i32,
    pub interaction_type: CommentInteractionType,
}

#[derive(Deserialize)]
pub struct CreateCommentInteractionPayload {
    pub user_id: i32,
}