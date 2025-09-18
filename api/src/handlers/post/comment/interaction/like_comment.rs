use crate::handlers::post::comment::interaction::common::create_interaction;
use crate::models::comment_struct::{CommentInteractionType, CreateCommentInteractionPayload};
use actix_web::{web, Responder};
use sqlx::PgPool;

/// Handler to like a comment.
pub async fn like_comment(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<CreateCommentInteractionPayload>,
) -> impl Responder {
    create_interaction(db_pool, path, payload, CommentInteractionType::Like).await
}