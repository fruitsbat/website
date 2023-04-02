use rocket::response::content::RawCss;

#[get("/index.css")]
pub fn css() -> RawCss<&'static str> {
    RawCss(grass::include!("src/css/index.scss"))
}
