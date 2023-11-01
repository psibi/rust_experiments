use chrono::{NaiveDate, NaiveTime, NaiveDateTime, FixedOffset, Utc};

fn main() {

    let date = NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();
    let time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

    let datetime = NaiveDateTime::new(date, time);
    let utc_time = datetime.and_utc();

    println!("Hello, world! {date:?} {time:?}");
    println!("Naive date time: {datetime:?}");
    println!("UTC time: {utc_time:?}");


}
