use actix_web::{web, HttpResponse, Responder};
use crate::models::post_struct::{Post, UpdatePostPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a post's content by its unique ID.
pub async fn update_post(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    payload: web::Json<UpdatePostPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Post>(
        r#"
        WITH updated AS (
            UPDATE post SET content = $1 WHERE id = $2 RETURNING *
        )
        SELECT * FROM updated
        "#,
    )
    .bind(&payload.content)
    .bind(post_id.into_inner())
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Post not found."})),
        Err(err) => {
            eprintln!("Database query error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update post."}))
        }
    }
}
