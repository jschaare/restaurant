use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::models::OrderTest;

#[post("/order")]
pub async fn create_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[get("/order")]
pub async fn get_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[delete("/order")]
pub async fn del_order(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}

#[get("/table")]
pub async fn get_table_orders(item: web::Json<OrderTest>) -> impl Responder {
    //format!("{:?}", item)
    HttpResponse::Ok().json(item)
}
