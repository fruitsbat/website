use rocket::response::content::RawCss;

lazy_static! {
    static ref CSS: RawCss<&'static str> = get_css();
}

#[get("/index.css")]
pub fn css() -> RawCss<&'static str> {
    CSS.to_owned()
}

pub fn get_css() -> RawCss<&'static str> {
    RawCss(grass::include!("src/css/index.scss"))
}
