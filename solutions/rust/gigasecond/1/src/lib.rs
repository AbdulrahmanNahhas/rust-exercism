use std::ops::Add;

use time::{Duration, PrimitiveDateTime as DateTime};

pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::seconds(1_000_000_000))
}
