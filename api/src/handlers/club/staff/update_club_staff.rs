use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::{ClubStaffInfo, UpdateClubStaffPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a club staff member's `promoted_by` field.
pub async fn update_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (club_id, user_id)
    payload: web::Json<UpdateClubStaffPayload>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = r#"
        UPDATE club_staff
        SET promoted_by = $1
        WHERE club_id = $2 AND user_id = $3
        RETURNING user_id, (SELECT username FROM users WHERE id = user_id) as username, promoted_by
    "#;

    match sqlx::query_as::<_, ClubStaffInfo>(query)
        .bind(payload.promoted_by)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(updated_member) => HttpResponse::Ok().json(updated_member),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found."}))
        }
        Err(e) => {
            eprintln!("Failed to update club staff member: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update staff member."}))
        }
    }
}