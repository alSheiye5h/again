use serde::{Deserialize, Serialize};

/// Represents a team in the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    pub id: i32,
    pub created_by: Option<i32>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTeamPayload {
    // Define fields that can be updated for a team
    pub description: Option<String>,
}


/// Payload for creating a new team.

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateTeamPayload {
    pub created_by: i32,
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

/// Pagination query parameters.
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Represents a team achievement.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamAchievement {
    pub id: i32,
    pub user_id: i32,
    pub team_id: i32,
}