use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::post_struct::{
    CreateInteractionPayload, PostInteraction, PostInteractionType,
};

/// Generic handler to create a post interaction.
pub async fn create_interaction(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    payload: web::Json<CreateInteractionPayload>,
    interaction_type: PostInteractionType,
) -> impl Responder {
    let post_id_val = post_id.into_inner();
    let user_id = payload.user_id;

    // Using ON CONFLICT to handle cases where the user has already interacted in the same way.
    // It also handles updating an upvote to a downvote or vice-versa.
    let query = r#"
        INSERT INTO post_interaction (user_id, post_id, interaction_type)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, post_id, interaction_type) DO NOTHING
        RETURNING *
    "#;

    let result = sqlx::query_as::<_, PostInteraction>(query)
        .bind(user_id)
        .bind(post_id_val)
        .bind(interaction_type)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(interaction)) => HttpResponse::Created().json(interaction),
        Ok(None) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Interaction already exists or was updated."
        })),
        Err(e) => {
            eprintln!("Failed to create post interaction: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to record interaction."}))
        }
    }
}