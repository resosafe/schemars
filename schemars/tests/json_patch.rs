mod util;
use json_patch::{Patch, PatchOperation};
use schemars::JsonSchema;
use util::*;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct ChronoTypes {
    weekday: Weekday,
    date_time: DateTime<Utc>,
    naive_date: NaiveDate,
    naive_date_time: NaiveDateTime,
    naive_time: NaiveTime,
}

#[test]
fn chrono_types() -> TestResult {
    test_default_generated_schema::<Patch>("chrono-types")
}
