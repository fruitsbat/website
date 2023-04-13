use rocket::http::ContentType;

#[get("/assets/fonts/atkinson.woff2")]
pub fn regular() -> (ContentType, &'static [u8]) {
    (ContentType::WOFF2, include_bytes!("atkinson_regular.woff2"))
}

#[get("/assets/fonts/atkinson_bold.woff2")]
pub fn bold() -> (ContentType, &'static [u8]) {
    (ContentType::WOFF2, include_bytes!("atkinson_bold.woff2"))
}

#[get("/assets/fonts/CascadiaCode.woff2")]
pub fn mono() -> (ContentType, &'static [u8]) {
    (ContentType::WOFF2, include_bytes!("CascadiaCode.woff2"))
}
