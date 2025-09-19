use actix_web::{web, HttpResponse, Responder};
use crate::models::post_struct::Post;
use serde_json::json;
use sqlx::PgPool;

/// Handler to retrieve all posts.
pub async fn list_posts(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Post>(
        r#"
        SELECT p.id, p.user_id, p.content, p.created_at, p.updated_at, pd.discussion_id
        FROM post p
        LEFT JOIN post_discussion pd ON p.id = pd.post_id
        ORDER BY p.created_at DESC
        "#,
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => {
            eprintln!("Database query error: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Database query failed."}))
        }
    }
}
