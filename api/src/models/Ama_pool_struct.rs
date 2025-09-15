use serde::{Deserialize, Serialize};
use sqlx::FromRow; // Make sure this is imported

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ama {
    pub id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct Ama_react {
    pub id: i32,
    pub user_id: i32,
    pub ama_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]

pub struct Pool {
    pub id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool_react {
    pub id: i32,
    pub user_id: i32,
    pub pool_id: i32,
}

#[derive(Deserialize)]
pub struct 
Create_community_ama_payload
 {
    pub created_by: i32,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct Club_community_ama {
    id: i32,
    community_id: i32,
    created_by: i32,
}