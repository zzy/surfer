use cookie::Cookie;
use time::Duration;

#[async_std::main]
async fn main() {
    let cookie = Cookie::build(
        "token", 
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6Im9rYTIyQGJ1ZHNob21lLmNvbSIsInVzZXJuYW1lIjoi5oiRMjJz6LCBMjRvazMyIiwiZXhwIjoxMDAwMDAwMDAwMH0.FUdYJeEL1eCfturVUoPYKaVG-m4e-Jl3YJviYg1b8O9hKw2rrH7HKZED0gDT4i5lKbI9VTfbI0Qu4Tt3apwpOw")
        .domain("budshome.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::minutes(60))
        .finish();
    println!("{:#?}\n", cookie);
    println!("{:#?}\n", cookie.name_value());
}
