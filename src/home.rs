use crate::{
    blinkies::Blinkybox,
    page::{Category, Page},
};
use maud::{html, Render};
use rocket::response::content::RawHtml;

#[get("/")]
pub fn home_page() -> RawHtml<String> {
    let content = html! {
        iframe
        frameBorder = "0"
        src="https://ring.bicompact.space/zoe-bat/pre"
        title="links from the armisael webring"
        {}
        (Blinkybox)
    };
    let page = Page {
        content,
        category: Category::Home,
        title: "home",
        path: vec!["/"],
    };
    RawHtml(page.render().into_string())
}
