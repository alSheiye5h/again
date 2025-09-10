use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]

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