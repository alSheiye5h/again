use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use api::database::connect_to_db;
use api::models::jwt_struct::Keys;
use api::routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    // std::env::set_var("RUST_LOG", "actix_web=debug"); // Uncomment for quick debug
    // std::env::set_var("RUST_BACKTRACE", "1"); // Optional: for backtraces on panics
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    // Initialize database pool
    let db_pool = connect_to_db::connect_db().await;
    println!("Database connected successfully");
    let db_pool_data = web::Data::new(db_pool);

    // Initialize JWT Keys from environment variable
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let keys = Keys::new(&jwt_secret);
    let keys_data = web::Data::new(keys);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone()) // <-- pass pool to handlers
            .app_data(keys_data.clone()) // <-- pass JWT keys to handlers
            .configure(|cfg| {
                cfg.service(
                    web::scope("/api")
                        .configure(routes::auth::auth_routes)
                        .configure(routes::post::post_routes)
                        .configure(routes::club::club_routes)
                        .configure(routes::community::community_routes)
                        .configure(routes::announcement::announcement_routes)
                        .configure(routes::discussion::discussion_routes)
                        .configure(routes::event::event_routes)
                        .configure(routes::user::user_routes)
                );
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
