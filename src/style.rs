use cached::proc_macro::cached;
use rocket::response::content::RawCss;

#[cached]
#[get("/index.css")]
pub fn css() -> RawCss<&'static str> {
    RawCss(grass::include!("src/css/index.scss"))
}
