use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
}
