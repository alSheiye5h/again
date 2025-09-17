use serde::{Deserialize, Serialize};

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
