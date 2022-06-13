use actix_web::{middleware, App, HttpServer};

mod models;
mod routes;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::create_order)
            .service(routes::get_order)
            .service(routes::del_order)
            .service(routes::get_table_orders)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
