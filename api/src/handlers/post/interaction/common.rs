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

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    // If the new interaction is a vote, remove any conflicting vote first.
    if interaction_type == PostInteractionType::Upvote || interaction_type == PostInteractionType::Downvote {
        let opposite_interaction = if interaction_type == PostInteractionType::Upvote {
            PostInteractionType::Downvote
        } else {
            PostInteractionType::Upvote
        };

        if let Err(e) = sqlx::query("DELETE FROM post_interaction WHERE user_id = $1 AND post_id = $2 AND interaction_type = $3")
            .bind(user_id)
            .bind(post_id_val)
            .bind(opposite_interaction)
            .execute(&mut *tx)
            .await {
                eprintln!("Failed to remove conflicting vote: {:?}", e);
                return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update vote"}));
            }
    }

    // Insert the new interaction. `ON CONFLICT DO NOTHING` handles cases where the user clicks the same button twice.
    let result = sqlx::query_as::<_, PostInteraction>(
        r#"
            INSERT INTO post_interaction (user_id, post_id, interaction_type)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, post_id, interaction_type) DO NOTHING
            RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(post_id_val)
    .bind(interaction_type)
    .fetch_optional(&mut *tx)
    .await;

    match result {
        Ok(interaction_opt) => {
            if let Err(e) = tx.commit().await {
                eprintln!("Failed to commit transaction: {:?}", e);
                return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to save interaction"}));
            }
            HttpResponse::Created().json(interaction_opt)
        },
        Err(e) => {
            eprintln!("Failed to create post interaction: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to record interaction."}))
        }
    }
}