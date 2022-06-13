use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OrderCreate {
    item_name: String,
    table_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    id: i32,
    table_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TableInput {
    table_id: i32,
}

#[derive(Debug, Serialize)]
pub struct Order {
    id: i32,
    item_name: String,
    table_id: i32,
    cook_time: i32,
}

impl Order {
    pub fn from_create(input: OrderCreate, id: i32, cook_time: i32) -> Order {
        Order {
            id: id,
            item_name: input.item_name,
            table_id: input.table_id,
            cook_time: cook_time,
        }
    }
}
