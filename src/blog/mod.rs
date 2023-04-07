use chrono::{DateTime, FixedOffset, TimeZone};
use maud::{html, Markup, Render};
use rocket::{http::Status, response::content::RawHtml};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    assets::Asset,
    components::{
        linkbox::{Linkbox, LinkboxContainer},
        tag::Tag,
    },
    page::{Category, Page},
};

// get main page
#[get("/log")]
pub fn main_page() -> RawHtml<String> {
    let linkbox_container = LinkboxContainer {
        linkboxes: BlogEntries::iter()
            .map(|i| i.linkbox())
            .collect::<Vec<Linkbox>>(),
    };
    let main_page = Page {
        category: crate::page::Category::Blog,
        title: "weblog",
        content: html! {
            (linkbox_container)
        },
        show_tags: true,
    };
    RawHtml(main_page.render().into_string())
}

#[derive(EnumIter)]
pub enum BlogEntries {
    Doggies,
    Kitties,
}

impl BlogEntries {
    pub fn preview_image(&self) -> Option<Asset> {
        match self {
            Self::Doggies => None,
            Self::Kitties => Some(Asset::Kittyroll),
        }
    }
    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::Doggies => vec![Tag::Doggies, Tag::Animals],
            Self::Kitties => vec![Tag::Kitties, Tag::Animals],
        }
    }
    pub fn content(&self) -> Markup {
        match self {
            Self::Kitties => html! {p {("meow meow meow")}},
            _ => html! {},
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Doggies => "doggies",
            Self::Kitties => "kitties",
        }
    }
    pub fn slug(&self) -> &'static str {
        match self {
            _ => self.title(),
        }
    }

    pub fn date(&self) -> DateTime<FixedOffset> {
        let hour = 3600;
        let time = match self {
            Self::Kitties => (2022, 12, 2, 0, 0, 0),
            _ => (2013, 1, 1, 13, 0, 0),
        };
        FixedOffset::west_opt(1 * hour)
            .unwrap()
            .with_ymd_and_hms(time.0, time.1, time.2, time.3, time.4, time.5)
            .unwrap()
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Doggies => "doggies know nothing their heads are empty",
            Self::Kitties => "kitties know how to see a ghost (scary)",
        }
    }

    /// atom feed entry for this blogpost
    pub fn entry(&self) -> atom_syndication::Entry {
        atom_syndication::Entry {
            title: self.title().into(),
            id: format!("{}/log/{}", crate::URL, self.slug()),
            categories: self
                .tags()
                .iter()
                .map(|tag| tag.category())
                .collect::<Vec<atom_syndication::Category>>(),
            summary: Some(atom_syndication::Text {
                value: self.description().into(),
                lang: Some("en".into()),
                ..Default::default()
            }),
            content: Some(atom_syndication::Content {
                base: Some(format!("{}", crate::URL)),
                lang: Some("en".into()),
                src: Some(format!("{}/log/{}", crate::URL, self.slug())),
                content_type: Some("html".into()),
                ..Default::default()
            }),
            updated: self.date(),
            ..Default::default()
        }
    }

    pub fn linkbox(&self) -> Linkbox {
        Linkbox {
            legend: self.title().into(),
            path: format!("/log/{}", self.slug()),
            image: self.preview_image(),
            description: self.description().into(),
            tags: self.tags(),
        }
    }
}

fn get_entry(entry: &str) -> Result<BlogEntries, Status> {
    for blog in BlogEntries::iter() {
        if blog.slug() == entry {
            return Ok(blog);
        }
    }
    Err(Status::NotFound)
}

#[get("/log/<entry>")]
pub fn pages(entry: String) -> Result<RawHtml<String>, Status> {
    match get_entry(&entry) {
        Err(status) => return Err(status),
        Ok(post) => {
            return Ok(RawHtml(
                Page {
                    content: post.content(),
                    title: post.title(),
                    show_tags: false,
                    category: Category::Blog,
                }
                .render()
                .into_string(),
            ))
        }
    }
}
