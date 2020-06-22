use chrono::NaiveDateTime;

pub fn encode_datetime(dt: NaiveDateTime) -> String {
    format!["{:?}", dt]
}
