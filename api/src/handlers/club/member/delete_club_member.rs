use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a member from a club.
pub async fn remove_member(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (club_id, user_id) = path.into_inner();

    match sqlx::query("DELETE FROM club_members WHERE club_id = $1 AND user_id = $2")
        .bind(club_id)
        .bind(user_id)
        .execute(&**db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound()
                    .json(json!({"status": "error", "message": "Member not found in this club."}))
            } else {
                HttpResponse::Ok()
                    .json(json!({"status": "success", "message": "Member removed from club successfully."}))
            }
        }
        Err(e) => {
            eprintln!("Failed to remove club member: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to remove member from club."}))
        }
    }
}
