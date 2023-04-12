use rocket::response::content::RawHtml;

#[get("/google8d7f2a3077574f63.html")]
pub fn google() -> RawHtml<&'static str> {
    RawHtml(include_str!("./google8d7f2a3077574f63.html"))
}
