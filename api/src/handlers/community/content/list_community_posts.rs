use crate::models::post_struct::Post;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

pub async fn list_community_posts(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let community_id = path.into_inner();

    // First, check if the community exists
    let community_exists: (bool,) = match sqlx::query_as("SELECT EXISTS(SELECT 1 FROM community WHERE id = $1)")
        .bind(community_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(exists) => exists,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check community existence."})),
    };

    if !community_exists.0 {
        return HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found."}));
    }

    match sqlx::query_as::<_, Post>(
        r#"
        SELECT p.*
        FROM post p
        JOIN community_post cp ON p.id = cp.post_id
        WHERE cp.community_id = $1
        "#,
    )
    .bind(community_id)
    .fetch_all(&**db_pool)
    .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Failed to list community posts: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list posts for the community."}))
        }
    }
}