use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::post_struct::{PostInteraction, PostInteractionType};

/// Generic handler to retrieve all interactions of a certain type for a post.
pub async fn get_interactions(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    interaction_type: PostInteractionType,
) -> impl Responder {
    let post_id_val = post_id.into_inner();

    let result = sqlx::query_as::<_, PostInteraction>(
        "SELECT * FROM post_interaction WHERE post_id = $1 AND interaction_type = $2",
    )
    .bind(post_id_val)
    .bind(interaction_type)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(interactions) => HttpResponse::Ok().json(interactions),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "No interactions of this type found for the post."
            }))
        }
        Err(e) => {
            eprintln!("Failed to fetch post interactions: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to retrieve interactions."}))
        }
    }
}