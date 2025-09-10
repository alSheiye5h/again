use actix_web::{App, HttpServer, web};
mod handlers;
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
            .configure(routes::route::auth_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
