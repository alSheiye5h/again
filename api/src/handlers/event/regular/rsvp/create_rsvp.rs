use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::Rsvp_struct::RsvpPayload;

/// Handler to create or update an RSVP for a tournament event.
pub async fn create_or_update_rsvp(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // event_id
    payload: web::Json<RsvpPayload>,
) -> impl Responder {
    let event_id = path.into_inner();

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to start transaction: {}", e)})),
    };

    // 1. Find the ID of the RSVP option from the provided text.
    let rsvp_option_id = match sqlx::query_scalar::<_, i32>(
        "SELECT id FROM regular_event_rsvp_config WHERE event_id = $1 AND option_text = $2"
    )
    .bind(event_id)
    .bind(&payload.rsvp)
    .fetch_optional(&mut *tx)
    .await {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::BadRequest().json(json!({"status": "error", "message": "Invalid RSVP option. Please ensure the RSVP option is valid for this event."})),
        Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database error while validating RSVP option: {}", e)})),
    };

    // 2. Insert or update the user's RSVP using the found ID.
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO regular_event_rsvp (user_id, event_id, rsvp)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, event_id) DO UPDATE SET rsvp = EXCLUDED.rsvp
        "#,
    )
    .bind(payload.user_id)
    .bind(event_id)
    .bind(rsvp_option_id)
    .execute(&mut *tx)
    .await {
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to record RSVP: {}", e)}));
    }

    // 3. Commit the transaction.
    match tx.commit().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "RSVP recorded successfully."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to commit transaction: {}", e)})),
    }
}
