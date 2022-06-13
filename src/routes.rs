use actix_web::{delete, get, post, web, HttpResponse, Responder};
use rand::Rng;
use sqlx::PgPool;

use crate::{
    db::{delete_order, find_all_orders, find_order, insert_order},
    models::{OrderCreate, OrderInput, TableInput},
    utils::get_end_time,
};

#[post("/order")]
pub async fn create_order(item: web::Json<OrderCreate>, pool: web::Data<PgPool>) -> impl Responder {
    let item = item.into_inner();
    let mut rng = rand::thread_rng();
    let cook_time: i32 = rng.gen_range(5..15);
    let end_time: chrono::NaiveDateTime = get_end_time(cook_time, chrono::Local::now().naive_utc());

    match insert_order(&pool, item.item_name, item.table_id, cook_time, end_time).await {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::InternalServerError().json("Unable to insert item"),
    }
}

#[get("/order")]
pub async fn get_order(item: web::Json<OrderInput>, pool: web::Data<PgPool>) -> impl Responder {
    let item = item.into_inner();
    match find_order(&pool, item.id, item.table_id).await {
        Some(order) => HttpResponse::Ok().json(order),
        None => HttpResponse::InternalServerError().json(format!(
            "Unable to find order with {} at table {}",
            item.id, item.table_id,
        )),
    }
}

#[delete("/order")]
pub async fn del_order(item: web::Json<OrderInput>, pool: web::Data<PgPool>) -> impl Responder {
    let item = item.into_inner();
    match delete_order(&pool, item.id, item.table_id).await {
        Some(res) => HttpResponse::Ok().json(res),
        None => HttpResponse::InternalServerError().json(format!(
            "Unable to delete order with {} at table {}",
            item.id, item.table_id,
        )),
    }
}

#[get("/table")]
pub async fn get_table_orders(
    item: web::Json<TableInput>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let item = item.into_inner();
    let time_now = chrono::Local::now().naive_utc();
    let table_orders = find_all_orders(&pool, item.table_id, time_now).await;
    HttpResponse::Ok().json(table_orders)
}
