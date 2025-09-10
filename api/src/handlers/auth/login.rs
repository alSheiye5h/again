use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Input from the user
#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Database user representation
#[derive(Debug, sqlx::FromRow)]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub email: Option<String>, // Nullable in DB
    pub password: String,
}

pub async fn login_user(
    db_pool: web::Data<PgPool>,
    user: web::Json<UserLogin>,
) -> impl Responder {
    let user_data = user.into_inner();

    // Input validation
    if user_data.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Password is required");
    }

    let has_username = !user_data.username.trim().is_empty();
    let has_email = !user_data.email.trim().is_empty();

    if (has_username && has_email) || (!has_username && !has_email) {
        return HttpResponse::BadRequest()
            .body("Provide either username or email, not both / neither");
    }

    // Query user
    let query_result: Result<Option<DbUser>, sqlx::Error> = if has_username {
        sqlx::query_as!(
            DbUser,
            "SELECT id, username, email, password FROM users WHERE username = $1",
            user_data.username
        )
        .fetch_optional(db_pool.get_ref())
        .await
    } else {
        sqlx::query_as!(
            DbUser,
            "SELECT id, username, email, password FROM users WHERE email = $1",
            user_data.email
        )
        .fetch_optional(db_pool.get_ref())
        .await
    };

    // Handle result
    match query_result {
        Ok(Some(record)) => {
            if record.password == user_data.password {
                HttpResponse::Ok().json(json!({
                    "status": "success",
                    "user_id": record.id,
                    "username": record.username,
                    "email": record.email.unwrap_or_default()
                }))
            } else {
                HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "Incorrect password"
                }))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User not found"
        })),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Database error"
            }))
        }
    }
}
