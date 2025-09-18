use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[serde(default)] // Allows deserializing from JSON without an `id` for creation
    pub id: i32,
    pub content: String,
    pub created_by: i32,
    pub has_discussion: bool,
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum PostInteractionType {
    Like,
    Upvote,
    Downvote,
    Repost,
    Share,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostInteraction {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub interaction_type: PostInteractionType,
}

#[derive(Deserialize)]
pub struct CreateInteractionPayload {
    pub user_id: i32,
}