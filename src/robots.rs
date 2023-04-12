#[get("/robots.txt")]
pub fn robots() -> &'static str {
    "User-agent: *\nDisallow:"
}
