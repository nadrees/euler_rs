use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};

fn main() {
    let mut date = Utc.ymd(1901, 01, 01);
    while date.weekday() != Weekday::Sun {
        date += Duration::days(1);
    }

    let limit = Utc.ymd(2000, 12, 31);
    let mut count = 0;
    while date <= limit {
        if date.day() == 1 {
            count += 1;
        }
        date += Duration::weeks(1);
    }

    println!("{}", count);
}
