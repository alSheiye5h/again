use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json;

/// Handler to delete a club by its ID.
pub async fn delete_club(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This will be the club_id from the path
) -> impl Responder {
    let club_id = path.into_inner(); // e.g., /club/123 -> club_id is 123

    // Start a transaction to ensure atomicity.
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Delete associated staff members. This is safe even if there are none.
    if let Err(e) = sqlx::query("DELETE FROM club_staff WHERE club_id = $1")
        .bind(club_id)
        .execute(&mut *tx)
        .await
    {
        eprintln!("Failed to delete club staff: {:?}", e);
        return HttpResponse::InternalServerError().json("Failed to delete associated club staff.");
    }

    // Step 2: Delete the club itself.
    let delete_result = sqlx::query("DELETE FROM club WHERE id = $1")
        .bind(club_id)
        .execute(&mut *tx)
        .await;

    if let Ok(result) = delete_result {
        if result.rows_affected() == 0 {
            return HttpResponse::NotFound().json(json!({"status": "error", "message": format!("Club with id {} not found", club_id)}));
        }
    }

    // Step 3: Commit the transaction.
    match tx.commit().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Club deleted successfully."})),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to delete club.")
        }
    }
}
