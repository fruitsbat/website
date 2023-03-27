use maud::html;

use crate::{components::tag::TagList, page::Page};

pub fn tags_page() -> Page {
    Page {
        content: html! {(TagList {})},
        category: crate::page::Category::Blog,
        path: vec!["/", "blog/", "tags/"],
        title: "tags",
    }
}
