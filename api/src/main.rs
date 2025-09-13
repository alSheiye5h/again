use actix_web::{App, HttpServer, web};
mod handlers;
mod models;
mod routes;
use database::connect_to_db;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_pool = connect_to_db::connect_db().await;
    println!("Database connected successfully");

    let db_pool_data = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone()) // <-- pass pool to handlers
            .configure(routes::auth::auth_routes)
            .configure(|cfg| {
                cfg.service(
                    web::scope("/api")
                        .configure(routes::post::post_routes)
                        .configure(routes::club::club_routes)
                        .configure(routes::community::community_routes)
                        .configure(routes::announcement::announcement_routes)
                        .configure(routes::event::event_routes)
                );
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
