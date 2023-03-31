use crate::{
    blinkies::{Blinkies, Blinkybox},
    files::Writable,
    page::{Category, Page},
};
use maud::html;
use strum::IntoEnumIterator;

pub fn home_page() -> Vec<Box<dyn Writable>> {
    let mut writables: Vec<Box<dyn Writable>> = vec![];
    let content = html! {
        iframe
        frameBorder = "0"
        src="https://ring.bicompact.space/zoe-bat/pre"
        title="links from the armisael webring"
        {}
        (Blinkybox)
    };
    writables.append(&mut vec![Box::new(Page {
        content,
        category: Category::Home,
        title: "home",
        path: vec!["/"],
    })]);
    for blinky in Blinkies::iter() {
        writables.push(Box::new(blinky.asset()));
    }
    writables
}
