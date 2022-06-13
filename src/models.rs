use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderTest {
    item_name: String,
    table_id: i32,
}
