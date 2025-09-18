use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::comment_struct::{
    CommentInteraction, CommentInteractionType, CreateCommentInteractionPayload,
};

/// Generic handler to create a comment interaction.
pub async fn create_interaction(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (post_id, comment_id)
    payload: web::Json<CreateCommentInteractionPayload>,
    interaction_type: CommentInteractionType,
) -> impl Responder {
    let (_post_id, comment_id) = path.into_inner();
    let user_id = payload.user_id;

    let query = r#"
        INSERT INTO post_comment_interactions (user_id, comment_id, interaction_type)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, comment_id, interaction_type) DO NOTHING
        RETURNING *
    "#;

    let result = sqlx::query_as::<_, CommentInteraction>(query)
        .bind(user_id)
        .bind(comment_id)
        .bind(interaction_type)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(interaction)) => HttpResponse::Created().json(interaction),
        Ok(None) => HttpResponse::Ok().json(json!({"status": "success", "message": "Interaction already exists."})),
        Err(e) => {
            eprintln!("Failed to create comment interaction: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to record interaction."}))
        }
    }
}