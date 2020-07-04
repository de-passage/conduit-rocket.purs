use std::cmp::{max, min};

const LIMIT: i32 = 20;
const MAX_LIMIT: i32 = 500;

pub fn coerce_limit(limit: Option<i32>) -> i32 {
    min(max(1, limit.unwrap_or(LIMIT.into())), MAX_LIMIT)
}

pub fn coerce_offset(offset: Option<i32>) -> i32 {
    max(0, offset.unwrap_or(0))
}
