

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] // Remove this line to allow extra fields
pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub profil_pic: String,
    pub bio: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct UserLogin {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct UserAchievements {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRelationship {
    pub id: i32,
    pub follower: i32,
    pub followed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserState {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct UserTagsPost {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

