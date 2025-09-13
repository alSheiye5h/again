use serde::{Deserialize, Serialize};
use sqlx::FromRow; // Make sure this is imported

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub profil_pic: String,
    pub cover_pic: String,
    pub created_by: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateClubPayload {
    pub name: String,
    pub profil_pic: String,
    pub cover_pic: String,
    pub created_by: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateClubPayload {
    pub name: Option<String>,
    pub profil_pic: Option<String>,
    pub cover_pic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubFeed {
    pub id: i32,
    pub club_id: i32,
    pub name: String,
    pub bio: String,
    pub created_at: String,
    pub privacy_state: bool,
    pub rules: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubCommunityPool {
    pub id: i32,
    pub club_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubDiscussion {
    pub id: i32,
    pub head: i32,
    pub club_id: i32,
}

/// Represents a single row in the `club_members` table.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubMembers {
    pub id: i32,
    pub club_id: i32,
    pub user_id: i32,
}

/// Represents the detailed information for a club member, used for listing members.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ClubMemberInfo {
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

/// Represents the detailed information for a club staff member, used for listing staff.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ClubStaffInfo {
    pub user_id: i32,
    pub username: String,
    pub promoted_by: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubPool {
    pub id: i32,
    pub created_by: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubPost {
    pub id: i32,
    pub created_by: i32,
    pub discussion_id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubStaff {
    pub id: i32,
    pub user_id: i32,
    pub club_id: i32,
    pub promoted_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubStash {
    pub id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubAchievements {
    pub id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubAma {
    pub id: i32,
    pub created_by: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubCommunity {
    pub id: i32,
    pub club_id: i32,
    pub name: String,
    pub description: String,
    pub created_by: i32,
}

/// Payload for creating a new community.
#[derive(Deserialize)]
pub struct CreateCommunityPayload {
    pub name: String,
    pub description: String,
    pub created_by: i32,
}

/// Payload for updating an existing community.
#[derive(Deserialize)]
pub struct UpdateCommunityPayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Payload for adding a member to a club.
#[derive(Deserialize)]
pub struct ClubMemberPayload {
    pub user_id: i32,
}

/// Payload for adding a staff member to a club.
#[derive(Deserialize)]
pub struct AddClubStaffPayload {
    pub user_id: i32,
    pub promoted_by: i32, // The ID of the user who is promoting the new staff member.
}

/// Payload for creating a new pool in a club's community.
#[derive(Deserialize)]
pub struct CreateCommunityPoolPayload {
    pub created_by: i32,
}
