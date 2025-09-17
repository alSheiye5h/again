use crate::models::Announcement_struct::AnnouncementStruct;
use crate::models::Discussion_struct::CreateDiscussionAnnouncementPayload;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new announcement and link it to a discussion.
pub async fn create_discussion_announcement(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<CreateDiscussionAnnouncementPayload>,
) -> impl Responder {
    let discussion_id_val = discussion_id.into_inner();

    // Start a transaction to ensure both inserts are atomic.
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Database transaction failed."}));
        }
    };

    // Step 1: Create the new announcement.
    let announcement_result = sqlx::query_as::<_, AnnouncementStruct>(
        "INSERT INTO announcements (title, content, created_by) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.created_by)
    .fetch_one(&mut *tx)
    .await;

    let announcement = match announcement_result {
        Ok(ann) => ann,
        Err(e) => {
            eprintln!("Failed to create announcement: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to create announcement."}));
        }
    };

    // Step 2: Link the new announcement to the discussion.
    let link_result = sqlx::query(
        "INSERT INTO discussion_announcements (discussion_id, announcement_id) VALUES ($1, $2)",
    )
    .bind(discussion_id_val)
    .bind(announcement.id)
    .execute(&mut *tx)
    .await;

    match link_result {
        Ok(_) => match tx.commit().await {
            Ok(_) => HttpResponse::Created().json(announcement),
            Err(e) => {
                eprintln!("Failed to commit transaction: {:?}", e);
                HttpResponse::InternalServerError()
                    .json(json!({"status": "error", "message": "Failed to save link."}))
            }
        },
        Err(e) => {
            eprintln!("Failed to link discussion announcement: {:?}", e);
            // The transaction will be rolled back automatically.
            HttpResponse::InternalServerError().json(
                json!({"status": "error", "message": "Failed to link announcement. Ensure discussion exists."}),
            )
        }
    }
}
