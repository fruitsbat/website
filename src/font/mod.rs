use rocket::http::ContentType;

#[get("/assets/fonts/atkinson.woff2")]
pub fn regular() -> (ContentType, &'static [u8]) {
    (ContentType::WOFF2, include_bytes!("atkinson_regular.woff2"))
}
