use cookie::Cookie;
use time::{Duration, OffsetDateTime};
use chrono::{Utc, Local, TimeZone};

#[async_std::main]
async fn main() {
    let cookie = Cookie::build(
        "token", 
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6Im9rYTIyQGJ1ZHNob21lLmNvbSIsInVzZXJuYW1lIjoi5oiRMjJz6LCBMjRvazMyIiwiZXhwIjoxMDAwMDAwMDAwMH0.FUdYJeEL1eCfturVUoPYKaVG-m4e-Jl3YJviYg1b8O9hKw2rrH7HKZED0gDT4i5lKbI9VTfbI0Qu4Tt3apwpOw")
        .domain("budshome.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .expires(OffsetDateTime::now_utc())
        .max_age(Duration::minutes(60))
        .finish();
    println!("{:#?}\n", cookie);
    println!("{:#?}\n", cookie.name_value());

    let now = Local::now();
    println!("{}", &now);
    println!("{}", Utc::now());

    let dt = Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 1, 444);
    let dt = Utc.ymd(2001, 9, 9).and_hms_milli(1, 46, 40, 555);

    println!("{:?}", OffsetDateTime::now_utc());
}
