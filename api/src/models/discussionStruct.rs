use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Discussion {
    pub id: i32,
    pub admin: i32,
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
