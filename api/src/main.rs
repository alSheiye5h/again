use actix_web::{web, App, HttpServer};
mod handlers;
mod routes; // import the routes folder
use database::connect_to_db; // <- now it works


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_pool = connect_to_db::connect_db().await;
    println!("Database connected successfully");

    HttpServer::new(|| {
        App::new()
            .configure(routes::route::auth_routes) // use routes module for route registration
    })
    .bind(("127.0.0.1", 8080))? // bind server to localhost:8080
    .run()
    .await
}
