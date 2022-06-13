use actix_web::{delete, get, post, web, HttpResponse, Responder};
use rand::Rng;

use crate::models::{Order, OrderCreate, OrderInput, TableInput};

#[post("/order")]
pub async fn create_order(item: web::Json<OrderCreate>) -> impl Responder {
    let item = item.into_inner();
    let mut rng = rand::thread_rng();
    let cook_time: i32 = rng.gen_range(5..15);
    HttpResponse::Ok().json(Order::from_create(item, 0, cook_time))
}

#[get("/order")]
pub async fn get_order(_item: web::Json<OrderInput>) -> impl Responder {
    HttpResponse::Ok().json({})
}

#[delete("/order")]
pub async fn del_order(_item: web::Json<OrderInput>) -> impl Responder {
    HttpResponse::Ok().json({})
}

#[get("/table")]
pub async fn get_table_orders(_item: web::Json<TableInput>) -> impl Responder {
    HttpResponse::Ok().json({})
}
