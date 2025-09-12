use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use serde_json::Value as JsonValue;
use sqlx::Type;


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



/// Represents the role of a user within a community.
/// The `#[sqlx(type_name = "member_role")]` macro is crucial for mapping this Rust enum
/// to a PostgreSQL ENUM type. Make sure you have a `CREATE TYPE member_role AS ENUM (...)`
/// in your database schema.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type, PartialEq)]
#[sqlx(type_name = "member_role", rename_all = "lowercase")]
pub enum MemberRole {
    Member,
    Staff,
    Admin,
}

/// Represents a member in the `community_member` join table.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CommunityMember {
    pub community_id: i32,
    pub user_id: i32,
    pub role: MemberRole,
    pub joined_at: chrono::NaiveDateTime,
}

/// Payload for adding a new member to a community.
#[derive(Deserialize)]
pub struct AddMemberPayload {
    pub user_id: i32,
}

/// Payload for updating a member's role.
#[derive(Deserialize)]
pub struct UpdateMemberPayload {
    pub role: MemberRole,
}
