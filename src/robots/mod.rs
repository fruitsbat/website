#[get("/robots.txt")]
pub fn robots() -> &'static str {
    include_str!("robots.txt")
}
