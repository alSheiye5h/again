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

    let result = sqlx::query(
        r#"
        INSERT INTO tournament_event_rsvp (user_id, event_id, rsvp)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, event_id) DO UPDATE SET rsvp = EXCLUDED.rsvp
        "#,
    )
    .bind(payload.user_id)
    .bind(event_id)
    .bind(payload.rsvp)
    .execute(&**db_pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "RSVP recorded successfully."})),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some("23503".into()) { // foreign_key_violation
                    return HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "Invalid RSVP option. Please ensure the RSVP option is valid for this event."
                    }));
                }
            }
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to record RSVP: {}", e)}))
        },
    }
}
