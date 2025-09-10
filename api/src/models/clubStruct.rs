use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub profil_pic: String,
    pub cover_pic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubFeed {
    pub id: i32,
    pub club_id: i32,
    pub name: String,
    pub bio: String,
    pub created_at: String,
    pub privacy_state: bool,
    pub rules: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubCommunityPool {
    pub id: i32,
    pub club_id: i32,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubDiscussion {
    pub id: i32,
    pub head: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubMembers {
    pub id: i32,
    pub club_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubPool {
    pub id: i32,
    pub created_by: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubPost {
    pub id: i32,
    pub created_by: i32,
    pub discussion_id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubStaff {
    pub id: i32,
    pub user_id: i32,
    pub club_id: i32,
    pub promoted_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubStash {
    pub id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubAchievements {
    pub id: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubAma {
    pub id: i32,
    pub created_by: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubCommunity {
    pub id: i32,
    pub created_by: i32,
    pub club_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubCommunityAma {
    pub id: i32,
    pub created_by: i32,
}