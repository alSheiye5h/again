use actix_web::{web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use sqlx::PgPool;
use bcrypt::verify;
use crate::models::user_struct::UserLogin;
use crate::models::user_struct::DbUser;
use crate::jwt::create_jwt;
use crate::models::jwt_struct::Keys;

pub async fn login_user(
    db_pool: web::Data<PgPool>,
    keys: web::Data<Keys>,
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

    if has_username && has_email {
        return HttpResponse::BadRequest().body("Provide either username or email, not both");
    }

    let query_result = if has_username {
        sqlx::query_as::<_, DbUser>("SELECT id, username, email, password FROM users WHERE username = $1")
            .bind(&user_data.username)
            .fetch_optional(db_pool.get_ref())
            .await
    } else if has_email {
        sqlx::query_as::<_, DbUser>("SELECT id, username, email, password FROM users WHERE email = $1")
            .bind(&user_data.email)
            .fetch_optional(db_pool.get_ref())
            .await
    } else {
        return HttpResponse::BadRequest().body("Either username or email is required");
    };

    let user = match query_result {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().body("User not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database query failed"),
    };

    // Verify password hash
    let valid_password = match verify(&user_data.password, &user.password) {
        Ok(valid) => valid,
        Err(_) => {
            // Log the error but return a generic failure message to the user
            eprintln!("Password verification error for user: {}", user.id);
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
    };

    if valid_password {
        // Create JWT
        let token = match create_jwt(&user.id.to_string(), keys.get_ref()) {
            Ok(t) => t,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to create token"),
        };

        let cookie = Cookie::build("j_lg_ui", token)
            .path("/")
            .http_only(true)
            .finish();

        HttpResponse::Ok()
            .cookie(cookie)
            .json(user)
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}