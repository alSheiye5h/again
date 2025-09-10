#[derive(Debug, Serialize, Deserialize)]

pub struct TournamentEvent {
    pub id: i32,
    pub club_host: i32,
    pub community_host: i32,
    pub organizer: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TournamentEventRsvp {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub rsvp: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TournamentRsvp {
    pub id: i32,
}