use chrono::{NaiveDate, Utc};
use chrono_tz::Europe::Warsaw;

pub fn days_of_community_existence() -> i64 {
    let formation = NaiveDate::from_ymd_opt(2020, 6, 7).unwrap();
    let today = Utc::now().with_timezone(&Warsaw).date_naive();

    today.signed_duration_since(formation).num_days()
}
