use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete a user by ID.
pub async fn delete_user(db_pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    // This performs a "soft delete" by setting the `deleted` flag to true.
    match sqlx::query!("UPDATE users SET deleted = true WHERE id = $1", *user_id)
        .execute(&**db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok()
                    .json(json!({"status": "success", "message": "User marked as deleted."}))
            } else {
                HttpResponse::NotFound().json(
                    json!({"status": "error", "message": format!("User with id {} not found.", *user_id)}),
                )
            }
        }
        Err(err) => {
            eprintln!("Failed to mark user as deleted: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to mark user as deleted."}))
        }
    }
}