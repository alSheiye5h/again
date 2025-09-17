use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete a discussion and all its messages.
pub async fn delete_discussion(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let id = discussion_id.into_inner();

    // A transaction is used to ensure that if any part of the deletion fails,
    // the entire operation is rolled back.
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to start transaction."}));
        }
    };

    // The `ON DELETE CASCADE` on the `discussion_message` table should handle deleting messages.
    // We just need to delete the discussion itself.
    let result = sqlx::query("DELETE FROM discussion WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => match tx.commit().await {
            Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Discussion deleted successfully."})),
            Err(e) => {
                eprintln!("Failed to commit transaction: {:?}", e);
                HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to commit transaction."}))
            }
        },
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Discussion not found."})),
        Err(e) => {
            eprintln!("Failed to delete discussion: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete discussion."}))
        }
    }
}
