use actix_web::{web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use sqlx::PgPool;
use api::models::userStruct::UserLogin;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
struct DbUser {
    id: i32,
    username: String,
    email: String,
    password: String,
}

pub async fn login_user(
    db_pool: web::Data<PgPool>,
    user_data: web::Json<UserLogin>,
) -> impl Responder {
    if user_data.password.trim().is_empty() {
        return HttpResponse::BadRequest().body("Password is required");
    }

    let has_username = user_data.username
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);

    let has_email = user_data.email
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);

    match (has_username, has_email) {
        (true, false) | (false, true) => {
            let user_result: Result<Option<DbUser>, sqlx::Error> = if has_username {
                sqlx::query_as::<_, DbUser>(
                    "SELECT id, username, email, password FROM users WHERE username = $1",
                )
                .bind(&user_data.username)
                .fetch_optional(db_pool.get_ref())
                .await
            } else {
                sqlx::query_as::<_, DbUser>(
                    "SELECT id, username, email, password FROM users WHERE email = $1",
                )
                .bind(&user_data.email)
                .fetch_optional(db_pool.get_ref())
                .await
            };

            let user = match user_result {
                Ok(opt) => opt,
                Err(_) => return HttpResponse::InternalServerError().body("Database query failed"),
            };

            match user {
                Some(u) => {
                    if u.password == user_data.password {
                        // Set cookie
                        let cookie = Cookie::build("user_id", u.id.to_string())
                            .path("/")
                            .http_only(true)
                            .finish();

                        HttpResponse::Ok()
                            .cookie(cookie)
                            .json(u)
                    } else {
                        HttpResponse::Unauthorized().body("Invalid password")
                    }
                }
                None => HttpResponse::NotFound().body("User not found"),
            }
        }
        (true, true) => HttpResponse::BadRequest().body("Provide either username or email, not both"),
        (false, false) => HttpResponse::BadRequest().body("Either username or email is required"),
    }
}


