use crate::handlers::post::interaction::common::create_interaction;
use crate::models::post_struct::{CreateInteractionPayload, PostInteractionType};
use actix_web::{web, Responder};
use sqlx::PgPool;

/// Handler to downvote a post.
pub async fn downvote_post(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    payload: web::Json<CreateInteractionPayload>,
) -> impl Responder {
    create_interaction(db_pool, post_id, payload, PostInteractionType::Downvote).await
}