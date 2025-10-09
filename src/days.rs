use chrono::{NaiveDate, Utc};

pub fn days_of_community_existence() -> i64 {
    let formation = NaiveDate::from_ymd_opt(2020, 6, 7).unwrap();
    let today = Utc::now().date_naive();

    today.signed_duration_since(formation).num_days()
}
