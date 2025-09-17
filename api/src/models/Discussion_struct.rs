use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]

pub struct Discussion {
    pub id: i32,
    pub created_by: i32,
    pub bio: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DiscussionMessage {
    pub id: i32,
    pub discussion_id: i32,
    pub content: String,
    pub created_by: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

/// Represents the role of a user within a discussion.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type, PartialEq)]
#[sqlx(type_name = "member_role", rename_all = "lowercase")]
pub enum MemberRole {
    Member,
    Staff,
    Admin,
}

/// Payload for updating a member's role in a discussion.
#[derive(Deserialize)]
pub struct UpdateMemberRolePayload {
    pub role: MemberRole,
}


#[derive(Debug, Serialize, Deserialize)]

pub struct DiscussionAma {
    pub id: i32,
    pub created_by: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct DiscussionMembers {
    pub id: i32,
    pub user_id: i32,
    pub discussion: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct DiscussionPool {
    pub id: i32,
    pub created_by: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct DiscussionStaff {
    pub id: i32,
    pub user_id: i32,
    pub discussion: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct LowDiscussionMembers {
    pub id: i32,
    pub user_id: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct LowDiscussionStaff {
    pub id: i32,
    pub user_id: i32,
    pub discussion_id: i32,
}

#[derive(Deserialize)]
pub struct AddMemberPayload {
    pub user_id: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DiscussionMemberInfo {
    pub user_id: i32,
    pub username: String,
    pub role: MemberRole,
}


#[derive(Deserialize)]
pub struct CreateDiscussionPayload {
    pub created_by: i32,
    pub bio: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDiscussionPayload {
    pub bio: Option<String>,
}


#[derive(Deserialize)]
pub struct CreateDiscussionMessagePayload {
    pub content: String,
    pub created_by: i32,
}

#[derive(Deserialize)]
pub struct UpdateDiscussionMessagePayload {
    pub content: String,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DiscussionAnnouncement {
    pub id: i32,
    pub discussion_id: i32,
    pub title: String,
    pub content: String,
    pub created_by: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateDiscussionAnnouncementPayload {
    pub title: String,
    pub content: String,
    pub created_by: i32,
}

#[derive(Deserialize)]
pub struct UpdateDiscussionAnnouncementPayload {
    pub title: Option<String>,
    pub content: Option<String>,
}