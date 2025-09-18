use crate::handlers::post::interaction::common::create_interaction;
use crate::models::Post_struct::{CreateInteractionPayload, PostInteractionType};
use actix_web::{web, Responder};
use sqlx::PgPool;

/// Handler to repost a post.
pub async fn repost_post(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    payload: web::Json<CreateInteractionPayload>,
) -> impl Responder {
    create_interaction(db_pool, post_id, payload, PostInteractionType::Repost).await
}