use crate::{blog::BlogEntry, config::CONFIG, page::Page};
use cached::proc_macro::cached;
use maud::{html, Markup, Render};
use rocket::{http::Status, response::content::RawHtml};
use strum::{EnumIter, IntoEnumIterator};

use super::linkbox::LinkboxContainer;

#[cached]
#[get("/tag/<link>")]
pub fn tags(link: String) -> Result<RawHtml<String>, Status> {
    for tag in Tag::iter() {
        if tag.link() == link {
            let mut linkboxes = vec![];
            for blog in BlogEntry::iter() {
                if blog.tags().contains(&tag) {
                    linkboxes.push(blog.linkbox())
                }
            }
            let linkbox_container = LinkboxContainer { linkboxes };
            let page = Page {
                content: html! {(linkbox_container)},
                category: crate::page::Category::Blog,
                meow: None,
                title: tag.display_as(),
                keywords: tag.display_as().into(),
                description: format!("blogposts tagged with: {}", tag.display_as()),
                show_tags: true,
                canonical: format!("{}/log", CONFIG.base_url),
                has_code: false,
            };
            return Ok(RawHtml(page.render().into_string()));
        }
    }
    Err(Status::NotFound)
}

#[derive(EnumIter, PartialEq, Clone, FromFormField)]
pub enum Tag {
    ThingsIMade,
    Rust,
    Programming,
    Cyberspace,
    Emoji,
    Tutorial,
    Atmega32u4,
    CircuitPlayground,
    Embedded,
}

impl Tag {
    pub fn frontpage_name(&self) -> &'static str {
        match self {
            Self::Atmega32u4 => "the atmega32u4",
            Self::CircuitPlayground => "the circuit playground classic",
            _ => self.display_as(),
        }
    }

    pub fn link(&self) -> &'static str {
        match &self {
            Self::ThingsIMade => "things_i_made",
            Self::Cyberspace => "cyberspace",
            Self::Programming => "programming",
            Self::Emoji => "emoji",
            Self::Rust => "rust",
            Self::Tutorial => "tutorial",
            Self::Atmega32u4 => "atmega32u4",
            Self::CircuitPlayground => "circuit_playground",
            Self::Embedded => "embedded",
        }
    }
    pub fn display_as(&self) -> &'static str {
        match &self {
            Self::Programming => "programming",
            Self::ThingsIMade => "things i made",
            Self::Cyberspace => "cyberspace",
            Self::Emoji => "emoji",
            Self::Rust => "rust",
            Self::Atmega32u4 => "atmega32u4",
            Self::CircuitPlayground => "circuit playground",
            Self::Embedded => "embedded programming",
            Self::Tutorial => "tutorials",
        }
    }
    pub fn category(&self) -> atom_syndication::Category {
        atom_syndication::Category {
            term: self.link().into(),
            label: Some(self.display_as().into()),
            ..Default::default()
        }
    }
    pub fn a(&self) -> Markup {
        html! {a href=(format!("/tag/{}", self.link())){
            (self.display_as())
        }}
    }
}

impl Render for Tag {
    fn render(&self) -> Markup {
        let link = format!("/tag/{}", self.link());
        html! {
        a
        class="tag"
        href=(link)
        {(self.display_as())}}
    }
}

pub struct TagList;
impl Render for TagList {
    fn render(&self) -> Markup {
        html! {
            div class="tag-list" {
                @for tag in Tag::iter() {
                    (tag)
                }
            }
        }
    }
}
