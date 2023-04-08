use crate::{
    blinkies::Blinkybox,
    markdown::Markdown,
    page::{Category, Page},
};
use maud::{html, Render};
use rocket::response::content::RawHtml;

#[get("/")]
pub fn home_page() -> RawHtml<String> {
    let content = html! {
        p {
            ("hi! i'm zoe,  welcome to my website! ")
            a href="/index.xml"
            {
                ("an atom feed is available here.")
            }
        }
        (Markdown(include_str!("aboutme.md")))
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
        ..Default::default()
    };
    RawHtml(page.render().into_string())
}
