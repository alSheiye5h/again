use serde::{Deserialize, Serialize};

/// Represents the configuration for an RSVP on a regular event.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegularRsvpConfig {
    pub id: i32,
    pub content: String,
    pub event_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegularRsvp {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RsvpPayload {
    pub user_id: i32,
    pub rsvp: i32, // e.g., 1 for 'Yes', 0 for 'No', 2 for 'Maybe'
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserRsvp {
    user_id: i32,
    username: String,
    rsvp: i32,
}

#[derive(Deserialize)]
pub struct ConfigureRsvpPayload {
    pub content: String,
}
