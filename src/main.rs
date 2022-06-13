use actix_web::{middleware, web::Data, App, HttpServer};
use db::init_db;
use sqlx::postgres::PgPoolOptions;

mod db;
mod models;
mod routes;
mod utils;

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

#[cfg(test)]
pub mod tests {
    use crate::models::Order;
    use crate::utils::get_end_time;

    use super::*;
    use actix_web::http::{Method, StatusCode};
    use actix_web::test;
    use serde_json::json;
    use sqlx::Row;

    #[actix_rt::test]
    async fn create_item() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:postgres@localhost/restaurant_test?sslmode=disable")
            .await
            .expect("Failed to create pool");

        init_db(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .service(routes::create_order),
        )
        .await;

        let req = test::TestRequest::with_uri("/order")
            .method(Method::POST)
            .set_json(json!({
                "item_name": "test",
                "table_id": 1 as i32
            }))
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body: Order = test::read_body_json(res).await;
        assert_eq!(body.item_name, "test");
        assert_eq!(body.table_id, 1);

        sqlx::query("DELETE FROM orders WHERE table_id = 1")
            .execute(&pool)
            .await
            .expect("failed to delete");
    }

    #[actix_rt::test]
    async fn get_item() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:postgres@localhost/restaurant_test?sslmode=disable")
            .await
            .expect("Failed to create pool");

        init_db(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .service(routes::get_order),
        )
        .await;

        let end_time: chrono::NaiveDateTime = get_end_time(15, chrono::Local::now().naive_utc());

        let res_insert = sqlx::query(
            r#"
        INSERT INTO orders (item_name, table_id, cook_time, ending_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
        )
        .bind("test")
        .bind(2 as i32)
        .bind(15 as i32)
        .bind(end_time)
        .map(|row| {
            let id: i32 = row.get("id");
            id
        })
        .fetch_one(&pool)
        .await;

        let res_id = match res_insert {
            Ok(id) => id,
            Err(_) => panic!("not inserted"),
        };

        let req = test::TestRequest::with_uri("/order")
            .method(Method::GET)
            .set_json(json!({
                "id": res_id,
                "table_id": 2 as i32
            }))
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body: Order = test::read_body_json(res).await;
        assert_eq!(body.id, res_id);
        assert_eq!(body.item_name, "test");
        assert_eq!(body.table_id, 2);
        assert_eq!(body.cook_time, 15);

        sqlx::query("DELETE FROM orders WHERE table_id = 2")
            .execute(&pool)
            .await
            .expect("failed to delete");
    }

    #[actix_rt::test]
    async fn get_items() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:postgres@localhost/restaurant_test?sslmode=disable")
            .await
            .expect("Failed to create pool");

        init_db(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .service(routes::get_table_orders),
        )
        .await;

        let end_time: chrono::NaiveDateTime = get_end_time(15, chrono::Local::now().naive_utc());

        let _res_insert = sqlx::query(
            r#"
        INSERT INTO orders (item_name, table_id, cook_time, ending_at)
            VALUES ($1, $2, $3, $4), ($1, $2, $3, $4), ($1, $2, $3, $4)
            RETURNING id
        "#,
        )
        .bind("test")
        .bind(4 as i32)
        .bind(15 as i32)
        .bind(end_time)
        .execute(&pool)
        .await;

        let req = test::TestRequest::with_uri("/table")
            .method(Method::GET)
            .set_json(json!({
                "table_id": 4 as i32
            }))
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body: Vec<Order> = test::read_body_json(res).await;
        assert_eq!(body.len(), 3);

        sqlx::query("DELETE FROM orders WHERE table_id = 4")
            .execute(&pool)
            .await
            .expect("failed to delete");
    }

    #[actix_rt::test]
    async fn delete_item() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:postgres@localhost/restaurant_test?sslmode=disable")
            .await
            .expect("Failed to create pool");

        init_db(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .service(routes::del_order),
        )
        .await;

        let end_time: chrono::NaiveDateTime = get_end_time(15, chrono::Local::now().naive_utc());

        let res_insert = sqlx::query(
            r#"
        INSERT INTO orders (item_name, table_id, cook_time, ending_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
        )
        .bind("test")
        .bind(3 as i32)
        .bind(15 as i32)
        .bind(end_time)
        .map(|row| {
            let id: i32 = row.get("id");
            id
        })
        .fetch_one(&pool)
        .await;

        let res_id = match res_insert {
            Ok(id) => id,
            Err(_) => panic!("not inserted"),
        };

        let req = test::TestRequest::with_uri("/order")
            .method(Method::DELETE)
            .set_json(json!({
                "id": res_id,
                "table_id": 3 as i32
            }))
            .to_request();

        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let body: String = test::read_body_json(res).await;
        assert_eq!(body, "order deleted");

        sqlx::query("DELETE FROM orders WHERE table_id = 3")
            .execute(&pool)
            .await
            .expect("failed to delete");
    }
}
