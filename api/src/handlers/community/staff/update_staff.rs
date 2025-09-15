use actix_web::{web, HttpResponse, Responder};
use api::models::Communitie_struct::{CommunityStaffInfo, UpdateCommunityStaffPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a community staff member's `promoted_by` field.
pub async fn update_community_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateCommunityStaffPayload>,
) -> impl Responder {
    let (community_id, user_id) = path.into_inner();

    let query = r#"
        UPDATE community_staff SET promoted_by = $1
        WHERE community_id = $2 AND user_id = $3
        RETURNING (SELECT username FROM users WHERE id = $3), user_id, promoted_by
    "#;

    match sqlx::query_as::<_, CommunityStaffInfo>(query)
        .bind(payload.promoted_by)
        .bind(community_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_member) => HttpResponse::Ok().json(updated_member),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found."}))
        }
        Err(e) => {
            eprintln!("Failed to update community staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update staff member."}))
        }
    }
}