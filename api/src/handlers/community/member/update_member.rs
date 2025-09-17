use actix_web::{web, HttpResponse, Responder};
use crate::models::Communitie_struct::{CommunityMember, UpdateMemberPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a community member's role.
pub async fn update_community_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateMemberPayload>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let result = sqlx::query_as::<_, CommunityMember>(
        "UPDATE community_members SET role = $1 WHERE community_id = $2 AND user_id = $3 RETURNING *",
    )
    .bind(payload.role)
    .bind(community_id)
    .bind(user_id)
    .fetch_optional(&**db_pool)
    .await;

    match result {
        Ok(Some(updated_member)) => HttpResponse::Ok().json(updated_member),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Member not found."})),
        Err(e) => {
            eprintln!("Failed to update community member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update member."}))
        }
    }
}