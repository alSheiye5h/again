use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ama {
    pub id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct AmaReact {
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
pub struct PoolReact {
    pub id: i32,
    pub user_id: i32,
    pub pool_id: i32,
}

#[derive(Deserialize)]
pub struct 

CreateCommunityAmaPayload
 {
    pub created_by: i32,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct 
ClubCommunityAma {
    id: i32,
    community_id: i32,
    created_by: i32,
}