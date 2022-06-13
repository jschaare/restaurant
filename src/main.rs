use actix_web::{delete, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderTest {
    item_name: String,
    table_id: i32,
}

#[post("/order")]
async fn create_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[get("/order")]
async fn get_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[delete("/order")]
async fn del_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[get("/table")]
async fn get_table_orders(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(greet)
            .service(create_order)
            .service(get_order)
            .service(del_order)
            .service(get_table_orders)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
