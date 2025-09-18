use actix_web::{web, HttpResponse, Responder};
use crate::models::post_struct::{CreatePostPayload, Post};
use sqlx::PgPool;

/// Handler to create a new post.
/// If `discussion_id` is provided, it links the post to a discussion in a transaction.
pub async fn create_post(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreatePostPayload>,
) -> impl Responder {
    let has_discussion = payload.has_discussion.unwrap_or(false);
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Step 1: Insert the post
    let post_result = sqlx::query_as::<_, Post>(
        "INSERT INTO post (user_id, content) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.user_id)
    .bind(&payload.content)
    .fetch_one(&mut *tx)
    .await;
    
    let post = match post_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to insert post: {:?}", e);
            // Rollback is handled automatically when `tx` is dropped
            return HttpResponse::InternalServerError().json("Failed to create post");
        }
    };
    
    // Step 2: If requested, create a discussion and link it to the post
    if has_discussion {
        let discussion_id_result = sqlx::query_scalar::<_, i32>(
            "INSERT INTO low_discussion DEFAULT VALUES RETURNING id",
        ).fetch_one(&mut *tx).await;

        match discussion_id_result {
            Ok(discussion_id) => {
                if let Err(e) = sqlx::query("INSERT INTO post_discussion (post_id, discussion_id) VALUES ($1, $2)")
                    .bind(post.id)
                    .bind(discussion_id)
                    .execute(&mut *tx)
                    .await {
                        eprintln!("Failed to link post to discussion: {:?}", e);
                        return HttpResponse::InternalServerError().json("Failed to link discussion");
                    }
                // The discussion_id is already part of the post struct, but we need to update the DB
                sqlx::query("UPDATE post SET discussion_id = $1 WHERE id = $2").bind(discussion_id).bind(post.id).execute(&mut *tx).await.ok();
            },
            Err(e) => { 
                 eprintln!("Failed to create discussion: {:?}", e);
                 return HttpResponse::InternalServerError().json("Failed to create discussion");
            }
        }
    }

    // Step 3: Commit the transaction
    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(post),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to save post")
        }
    }
}
