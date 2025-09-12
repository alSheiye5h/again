use actix_web::{web, HttpResponse, Responder};
use api::models::eventStruct::{CreateEventPayload, Event};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new event.
pub async fn create_event(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreateEventPayload>,
) -> impl Responder {
    // Validate that exactly one of club_host or community_host is provided.
    if payload.club_host.is_some() == payload.community_host.is_some() {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "An event must have exactly one host: either a club_host or a community_host."
        }));
    }

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    let event_result = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO event (club_host, community_host, organizer, has_discussion)
        VALUES ($1, $2, $3, $4)
        RETURNING id, club_host, community_host, organizer, has_discussion, NULL as discussion_id
        "#,
    )
    .bind(payload.club_host)
    .bind(payload.community_host)
    .bind(payload.organizer)
    .bind(payload.has_discussion)
    .fetch_one(&mut *tx)
    .await;

    let mut event = match event_result {
        Ok(event) => event,
        Err(e) => {
            eprintln!("Failed to create event: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to create event."}));
        }
    };

    // If requested, create a discussion and link it to the event.
    if event.has_discussion {
        let discussion_id_result = sqlx::query_scalar::<_, i32>(
            "INSERT INTO low_discussion DEFAULT VALUES RETURNING id",
        )
        .fetch_one(&mut *tx)
        .await;

        match discussion_id_result {
            Ok(discussion_id) => {
                if let Err(e) = sqlx::query("INSERT INTO event_discussion (event_id, discussion_id) VALUES ($1, $2)")
                    .bind(event.id)
                    .bind(discussion_id)
                    .execute(&mut *tx)
                    .await {
                        eprintln!("Failed to link event to discussion: {:?}", e);
                        return HttpResponse::InternalServerError().json("Failed to link discussion");
                    }
                event.discussion_id = Some(discussion_id);
            },
            Err(e) => {
                eprintln!("Failed to create discussion: {:?}", e);
                return HttpResponse::InternalServerError().json("Failed to create discussion");
            }
        }
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(event),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create event.")
        }
    }
}