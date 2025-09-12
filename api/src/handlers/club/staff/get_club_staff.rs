use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::ClubStaffInfo;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific staff member from a club.
pub async fn get_staff(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    let query = "
        SELECT cs.user_id, u.username, cs.promoted_by
        FROM club_staff cs
        JOIN users u ON cs.user_id = u.id
        WHERE cs.club_id = $1 AND cs.user_id = $2
    ";

    match sqlx::query_as::<_, ClubStaffInfo>(query)
        .bind(club_id)
        .bind(user_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(staff) => HttpResponse::Ok().json(staff),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Staff member not found in this club."})),
        Err(e) => {
            eprintln!("Failed to fetch club staff: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch club staff."}))
        }
    }
}
