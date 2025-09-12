use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub club_host: Option<i32>,
    pub community_host: Option<i32>,
    pub organizer: i32,
    pub has_discussion: bool,
    pub discussion_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateEventPayload {
    pub club_host: Option<i32>,
    pub community_host: Option<i32>,
    pub organizer: i32,
    pub has_discussion: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventPayload {
    pub club_host: Option<i32>,
    pub community_host: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize)]

pub struct CharityEvent {
    pub id: i32,
    pub club_host: i32,
    pub community_host: i32,
    pub organizer: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CharityEventDonation {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CharityEventRsvp {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub rsvp: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CharityEventVolunteers {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct CharityRsvp {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct EventAssistant {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct LowDiscussion {
    pub id: i32,
}


#[derive(Debug, Serialize, Deserialize)]

pub struct RegularEvent {
    pub id: i32,
    pub club_host: i32,
    pub community_host: i32,
    pub organizer: i32,
    pub discussion_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct RegularEventRsvp {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub rsvp: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct RegularRsvp {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct TournamentEvent {
    pub id: i32,
    pub club_host: i32,
    pub community_host: i32,
    pub organizer: i32,
    pub discussion_id: i32,
}