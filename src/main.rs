use actix_web::{middleware, web::Data, App, HttpServer};
use db::init_db;
use sqlx::postgres::PgPoolOptions;

mod db;
mod models;
mod routes;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:postgres@localhost/restaurant?sslmode=disable")
        .await
        .expect("Failed to create pool");

    init_db(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(routes::create_order)
            .service(routes::get_order)
            .service(routes::del_order)
            .service(routes::get_table_orders)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
