use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderCreate {
    pub item_name: String,
    pub table_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub id: i32,
    pub table_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TableInput {
    pub table_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub item_name: String,
    pub table_id: i32,
    pub cook_time: i32,
    pub ending_at: chrono::NaiveDateTime,
}

impl Order {
    pub fn new(
        id: i32,
        item_name: String,
        table_id: i32,
        cook_time: i32,
        ending_at: chrono::NaiveDateTime,
    ) -> Order {
        Order {
            id: id,
            item_name: item_name,
            table_id: table_id,
            cook_time: cook_time,
            ending_at: ending_at,
        }
    }
}
