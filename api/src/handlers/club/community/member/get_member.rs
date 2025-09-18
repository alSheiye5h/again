use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::ClubCommunityMember;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single member from a community.
pub async fn get_community_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = r#"
        SELECT ccm.id, ccm.user_id, ccm.club_community_id, ccm.role, ccm.joined_at
        FROM club_community_members ccm
        JOIN club_community cc ON ccm.club_community_id = cc.id
        WHERE cc.club_id = $1 AND ccm.user_id = $2
    "#;

    match sqlx::query_as::<_, ClubCommunityMember>(query)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(member) => HttpResponse::Ok().json(member),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found in this club's community."}))
        }
        Err(e) => {
            eprintln!("Failed to fetch community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch member."}))
        }
    }
}