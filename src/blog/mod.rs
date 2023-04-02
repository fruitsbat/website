use maud::{html, Markup, Render};
use rocket::response::content::RawHtml;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    assets::Asset,
    components::{
        linkbox::{Linkbox, LinkboxContainer},
        tag::Tag,
    },
    page::Page,
};

// get main page
#[get("/log")]
pub fn main_page() -> RawHtml<String> {
    let linkbox_container = LinkboxContainer {
        linkboxes: BlogEntries::iter()
            .map(|i| i.get())
            .map(|i| i.linkbox())
            .collect::<Vec<Linkbox>>(),
    };
    let main_page = Page {
        category: crate::page::Category::Blog,
        path: vec!["/", "blog/"],
        title: "title",
        content: html! {
            (linkbox_container)
        },
    };
    RawHtml(main_page.render().into_string())
}

pub struct BlogEntry {
    pub preview_image: Option<Asset>,
    pub tags: Vec<Tag>,
    pub content: Markup,
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

impl BlogEntry {
    pub fn linkbox(&self) -> Linkbox {
        Linkbox {
            legend: self.title.into(),
            path: vec!["/", "blog/", self.slug],
            image: self.preview_image,
            description: self.description.into(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(EnumIter)]
enum BlogEntries {
    Doggies,
    Kitties,
}

/// list of all the blog posts
impl BlogEntries {
    fn get(&self) -> BlogEntry {
        match &self {
            BlogEntries::Doggies => BlogEntry {
                preview_image: None,
                tags: vec![Tag::Doggies, Tag::Animals],
                content: html! {},
                slug: "doggies/",
                title: "doggies",
                description: "doggies know nothing they heads are empty",
            },
            BlogEntries::Kitties => BlogEntry {
                preview_image: Some(Asset::Kittyroll),
                tags: vec![Tag::Kitties, Tag::Animals, Tag::Meow],
                content: html! {},
                slug: "kitties/",
                title: "kitties",
                description: "kittes know how to see a ghost (scary)",
            },
        }
    }
}
