use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::ClubStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all staff members of a club.
pub async fn list_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    let query = "
        SELECT cs.user_id, u.username, cs.promoted_by
        FROM club_staff cs
        JOIN users u ON cs.user_id = u.id
        WHERE cs.club_id = $1
    ";

    match sqlx::query_as::<_, ClubStaffInfo>(query)
        .bind(club_id)
        .fetch_all(&**db_pool)
        .await {
        Ok(staff_list) => HttpResponse::Ok().json(staff_list),
        Err(e) => {
            eprintln!("Failed to list club staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list club staff."}))
        }
    }
}
