use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComminitieStruct {
    pub id: u32,
    pub name: String,
    pub content: String,
    pub target: String,
    pub created_by: String,
    pub created_at: String,
    pub updateda_t: String,
}