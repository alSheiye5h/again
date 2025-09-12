use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete a community from a club.
pub async fn delete_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (club_id, community_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM club_community WHERE club_id = $1 AND id = $2")
        .bind(club_id)
        .bind(community_id)
        .execute(&**db_pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Community deleted successfully."}))
        }
        Ok(_) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found in this club."}))
        }
        Err(e) => {
            eprintln!("Failed to delete community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete community."}))
        }
    }
}
