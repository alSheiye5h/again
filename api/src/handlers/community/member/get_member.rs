use actix_web::{web, HttpResponse, Responder};
use crate::models::Communitie_struct::{CommunityMember, MemberRole};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single member from a community.
pub async fn get_community_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let result = sqlx::query_as::<_, CommunityMember>(
        "SELECT * FROM community_members WHERE community_id = $1 AND user_id = $2 AND role = $3",
    )
    .bind(community_id)
    .bind(user_id)
    .bind(MemberRole::Member)
    .fetch_optional(&**db_pool)
    .await;

    match result {
        Ok(Some(member)) => HttpResponse::Ok().json(member),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found."})),
        Err(e) => {
            eprintln!("Failed to fetch community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch member."}))
        }
    }
}