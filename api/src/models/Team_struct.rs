use serde::{Deserialize, Serialize};

/// Represents a team in the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    pub id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Team {
    pub id: i32,
    pub created_by: i32,
}



/// Payload for creating a new team.
#[derive(Debug, Deserialize)]
pub struct CreateTeamPayload {
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TeamAchievement {
    pub id: i32,
    pub user_id: i32,
    pub team_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMember {
    pub id: i32,
    pub user_id: i32,
    pub team_id: i32,
}

/// Payload for adding a member to a team.
#[derive(Debug, Deserialize)]
pub struct AddTeamMemberPayload {
    pub user_id: i32,
}

/// Detailed information for a team member, useful for listing members.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TeamMemberInfo {
    pub user_id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TeamAchievement {
    pub id: i32,
    pub team_id: i32,
}
/// Pagination query parameters.
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TeamMembers {
    pub id: i32,
    pub user_id: i32,
    pub team_id: i32,
}

