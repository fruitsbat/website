use crate::{blog::BlogEntry, page::Page};
use maud::{html, Markup, Render};
use rocket::{http::Status, response::content::RawHtml};
use strum::{EnumIter, IntoEnumIterator};

use super::linkbox::LinkboxContainer;

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
                title: tag.display_as(),
                show_tags: true,
                ..Default::default()
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
}

impl Tag {
    pub fn link(&self) -> &'static str {
        match &self {
            Self::ThingsIMade => "things_i_made",
            Self::Cyberspace => "cyberspace",
            Self::Programming => "programming",
            Self::Emoji => "emoji",
            Self::Rust => "rust",
        }
    }
    pub fn display_as(&self) -> &'static str {
        match &self {
            Self::Programming => "programming",
            Self::ThingsIMade => "things i made",
            Self::Cyberspace => "cyberspace",
            Self::Emoji => "emoji",
            Self::Rust => "rust",
        }
    }
    pub fn category(&self) -> atom_syndication::Category {
        atom_syndication::Category {
            term: self.link().into(),
            label: Some(self.display_as().into()),
            ..Default::default()
        }
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
