pub fn get_end_time(cook_time: i32, now: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
    now + chrono::Duration::minutes(cook_time as i64)
}
