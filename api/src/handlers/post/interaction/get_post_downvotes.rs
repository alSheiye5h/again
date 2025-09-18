use actix_web::{web, Responder};
use sqlx::PgPool;

use crate::handlers::post::interaction::get_interactions::get_interactions;
use crate::models::post_struct::PostInteractionType;

/// Handler to get all downvotes for a post.
pub async fn get_post_downvotes(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    get_interactions(db_pool, post_id, PostInteractionType::Downvote).await
}