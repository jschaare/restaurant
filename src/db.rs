use sqlx::{PgPool, Row};

use crate::models::Order;

const CREATE_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    item_name TEXT NOT NULL,
    table_id INTEGER NOT NULL,
    cook_time INTEGER NOT NULL,
    ending_at TIMESTAMP NOT NULL
);
"#;

const INSERT_ORDER_SQL: &str = r#"
INSERT INTO orders (item_name, table_id, cook_time, ending_at)
    VALUES ($1, $2, $3, $4)
    RETURNING id, item_name, table_id, cook_time, ending_at
"#;

const FIND_ORDER_SQL: &str = r#"
SELECT * FROM orders WHERE id = $1 AND table_id = $2
"#;

const DELETE_ORDER_SQL: &str = r#"
DELETE FROM orders WHERE id = $1 AND table_id = $2
"#;

const FIND_ALL_ORDERS_SQL: &str = r#"
SELECT * FROM orders WHERE table_id = $1 AND ending_at >= $2
"#;

pub async fn init_db(pool: &PgPool) {
    let _tx = sqlx::query(CREATE_TABLE_SQL).execute(pool).await;
}

pub async fn insert_order(
    pool: &PgPool,
    item_name: String,
    table_id: i32,
    cook_time: i32,
    end_time: chrono::NaiveDateTime,
) -> Option<Order> {
    let tx = sqlx::query(INSERT_ORDER_SQL)
        .bind(item_name)
        .bind(table_id)
        .bind(cook_time)
        .bind(end_time)
        .map(|row| {
            Order::new(
                row.get("id"),
                row.get("item_name"),
                row.get("table_id"),
                row.get("cook_time"),
                row.get("ending_at"),
            )
        })
        .fetch_one(pool)
        .await;

    match tx {
        Ok(order) => Some(order),
        Err(_) => None,
    }
}

pub async fn find_order(pool: &PgPool, item_id: i32, table_id: i32) -> Option<Order> {
    let tx = sqlx::query(FIND_ORDER_SQL)
        .bind(item_id)
        .bind(table_id)
        .map(|row| {
            Order::new(
                row.get("id"),
                row.get("item_name"),
                row.get("table_id"),
                row.get("cook_time"),
                row.get("ending_at"),
            )
        })
        .fetch_one(pool)
        .await;

    match tx {
        Ok(order) => Some(order),
        Err(_) => None,
    }
}

pub async fn delete_order(pool: &PgPool, item_id: i32, table_id: i32) -> Option<String> {
    let tx = sqlx::query(DELETE_ORDER_SQL)
        .bind(item_id)
        .bind(table_id)
        .execute(pool)
        .await;

    match tx {
        Ok(_) => Some("order deleted".to_string()),
        Err(_) => None,
    }
}

pub async fn find_all_orders(
    pool: &PgPool,
    table_id: i32,
    time_now: chrono::NaiveDateTime,
) -> Vec<Order> {
    let tx = sqlx::query(FIND_ALL_ORDERS_SQL)
        .bind(table_id)
        .bind(time_now)
        .fetch_all(pool)
        .await;
    match tx {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                Order::new(
                    row.get("id"),
                    row.get("item_name"),
                    row.get("table_id"),
                    row.get("cook_time"),
                    row.get("ending_at"),
                )
            })
            .collect(),
        Err(_) => vec![],
    }
}
