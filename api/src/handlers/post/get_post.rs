use actix_web::{web, HttpResponse, Responder};
use api::models::postStruct::Post;
use serde_json::json;
use sqlx::PgPool;

/// Handler to retrieve a post by its unique ID.
pub async fn get_post_by_id(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Post>(
        r#"
        SELECT p.id, p.content, p.created_by, p.has_discussion, pd.discussion_id
        FROM post p
        LEFT JOIN post_discussion pd ON p.id = pd.post_id
        WHERE p.id = $1
        "#,
    )
    .bind(post_id.into_inner())
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Post not found."})),
        Err(err) => {
            eprintln!("Database query error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Database query failed."}))
        }
    }
}
