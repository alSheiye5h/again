use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete an event by its unique ID.
pub async fn delete_event(
    db_pool: web::Data<PgPool>,
    event_id: web::Path<i32>,
) -> impl Responder {
    let event_id_val = event_id.into_inner();

    // Start a transaction.
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Find the discussion_id associated with the event, if any.
    let discussion_id_result = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT discussion_id FROM regular_event_discussion WHERE event_id = $1",
    )
    .bind(event_id_val)
    .fetch_optional(&mut *tx)
    .await;

    // Step 2: Delete the event. Assuming `ON DELETE CASCADE` on the `event_discussion` table's foreign key.
    let event_delete_result = sqlx::query("DELETE FROM regular_event WHERE id = $1")
        .bind(event_id_val)
        .execute(&mut *tx)
        .await;

    if let Ok(result) = event_delete_result {
        if result.rows_affected() == 0 {
            return HttpResponse::NotFound().json(json!({"status": "error", "message": "Event not found."}));
        }
    }

    // Step 3: If a discussion existed, delete it from the `low_discussion` table.
    if let Ok(Some(Some(discussion_id))) = discussion_id_result {
        if let Err(e) = sqlx::query("DELETE FROM low_discussion WHERE id = $1")
            .bind(discussion_id)
            .execute(&mut *tx)
            .await {
                eprintln!("Failed to delete associated discussion: {:?}", e);
                return HttpResponse::InternalServerError().json("Failed to delete associated discussion.");
            }
    }

    // Step 4: Commit the transaction.
    match tx.commit().await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Event deleted successfully."
        })),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to delete event.")
        }
    }
}