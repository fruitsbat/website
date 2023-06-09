use cached::proc_macro::cached;
use chrono::{DateTime, FixedOffset, TimeZone};
use itertools::{join, Itertools};
use maud::{html, Markup, Render};
use rocket::{http::Status, response::content::RawHtml};
use sitewriter::UrlEntry;
use std::collections::HashSet;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    assets::Asset,
    components::{
        linkbox::{Linkbox, LinkboxContainer},
        meow::Meow,
        tag::Tag,
    },
    config::CONFIG,
    markdown::Markdown,
    page::{Category, Page},
};

// get main page
#[get("/log")]
#[cached]
pub fn main_page() -> RawHtml<String> {
    let linkbox_container = LinkboxContainer {
        linkboxes: BlogEntry::iter()
            .map(|i| i.linkbox())
            .collect::<Vec<Linkbox>>(),
    };
    let main_page = Page {
        category: crate::page::Category::Blog,
        canonical: format!("{}/log", CONFIG.base_url),
        meow: None,
        title: "weblog",
        content: html! {
            (linkbox_container)
        },
        show_tags: true,
        description: "a list of all blog entries about various topics".into(),
        // use all tags as keywords
        keywords: join(Tag::iter().map(|t| t.display_as()), ", "),
        has_code: false,
    };
    RawHtml(main_page.render().into_string())
}

#[derive(EnumIter, Clone, Copy, Eq, PartialEq)]
pub enum BlogEntry {
    RustAtmegaTutorial,
    Kaokao,
    RewritingMyWebsiteInRust,
}

impl BlogEntry {
    pub fn has_code(&self) -> bool {
        match self {
            Self::RustAtmegaTutorial => true,
            _ => false,
        }
    }

    pub fn preview_image(&self) -> Option<Asset> {
        match self {
            Self::Kaokao => Some(Asset::Kaokao),
            Self::RustAtmegaTutorial => Some(Asset::Blink),
            _ => None,
        }
    }
    pub fn tags(&self) -> HashSet<Tag> {
        match self {
            Self::RewritingMyWebsiteInRust => HashSet::from([
                Tag::Cyberspace,
                Tag::ThingsIMade,
                Tag::Programming,
                Tag::Rust,
            ]),
            Self::Kaokao => {
                HashSet::from([Tag::ThingsIMade, Tag::Emoji, Tag::Programming, Tag::Rust])
            }
            Self::RustAtmegaTutorial => HashSet::from([
                Tag::Rust,
                Tag::Atmega32u4,
                Tag::CircuitPlayground,
                Tag::Tutorial,
                Tag::Programming,
                Tag::Embedded,
            ]),
        }
    }
    pub fn content(&self) -> Markup {
        let content = match self {
            Self::RewritingMyWebsiteInRust => {
                Markdown(include_str!("./rewriting_my_website.md")).render()
            }
            Self::Kaokao => Markdown(include_str!("./kaokao.md")).render(),
            Self::RustAtmegaTutorial => {
                Markdown(include_str!("building_rust_code_for_atmega32u4.md")).render()
            }
        };

        let linkboxes = LinkboxContainer {
            linkboxes: self
                .similar()
                .iter()
                .map(|post| post.linkbox())
                .collect_vec(),
        };

        let content = html! {
            div class="blogpost" {
                (content)
            }
            h2 {"more like this:"}
            (linkboxes)
        };
        content
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::RewritingMyWebsiteInRust => "rewriting my website in rust",
            Self::Kaokao => "kaokao",
            Self::RustAtmegaTutorial => {
                "how to run rust code on a circuit playground classic / atmega32u4"
            }
        }
    }
    pub fn slug(&self) -> &'static str {
        match &self {
            Self::RewritingMyWebsiteInRust => "rewriting_my_website_in_rust",
            Self::Kaokao => "kaokao",
            Self::RustAtmegaTutorial => "rust_atmega32u4_tutorial",
        }
    }

    pub fn date(&self) -> DateTime<FixedOffset> {
        let hour = 3600;
        let time = match self {
            Self::RewritingMyWebsiteInRust => (2023, 4, 8, 23, 30, 0),
            Self::Kaokao => (2023, 4, 9, 10, 0, 0),
            Self::RustAtmegaTutorial => (2023, 4, 13, 22, 0, 0),
        };
        FixedOffset::west_opt(2 * hour)
            .unwrap()
            .with_ymd_and_hms(time.0, time.1, time.2, time.3, time.4, time.5)
            .unwrap()
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::RewritingMyWebsiteInRust => {
                "a reasonable and important thing to do for a personal blog"
            }
            Self::Kaokao => "kaokao is an emoji picker with support for custom and builtin kaomoji",
            Self::RustAtmegaTutorial => "a guide on how to set up a rust project for the circuit playground classic / atmega32u4"
        }
    }

    /// atom feed entry for this blogpost
    pub fn entry(&self) -> atom_syndication::Entry {
        atom_syndication::Entry {
            title: self.title().into(),
            id: format!("{}/log/{}", CONFIG.base_url, self.slug()),
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
                base: Some(CONFIG.base_url.to_string()),
                lang: Some("en".into()),
                src: Some(format!("{}/log/{}", CONFIG.base_url, self.slug())),
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
            tags: self.tags().into_iter().collect_vec(),
        }
    }

    pub fn url_entry(&self) -> UrlEntry {
        UrlEntry {
            loc: format!("{}/log/{}", CONFIG.base_url, self.slug())
                .parse()
                .unwrap(),
            lastmod: Some(self.date().into()),
            changefreq: None,
            priority: Some(0.7),
        }
    }

    /// sort all other blogposts by similarity
    pub fn similar(&self) -> Vec<BlogEntry> {
        let own_tag_hash = self.tags().into_iter().collect::<HashSet<_>>();
        Self::iter()
            .filter(|post| post != self)
            .map(|entry| {
                let weight = (100 as usize).div_floor(entry.tags().len());
                (
                    entry,
                    own_tag_hash
                        .intersection(&entry.tags().into_iter().collect::<HashSet<_>>())
                        .count()
                        * weight,
                )
            })
            .map(|(post, similarity)| {
                return (post, similarity);
            })
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .map(|(post, _similarity)| post)
            .collect::<Vec<BlogEntry>>()
    }
}

/// get blog entry by slug
pub fn get_entry(entry: &str) -> Result<BlogEntry, Status> {
    for blog in BlogEntry::iter() {
        if blog.slug() == entry {
            return Ok(blog);
        }
    }
    Err(Status::NotFound)
}

/// turn blog entries into pages and serve them
#[get("/log/<entry>")]
pub fn pages(entry: String) -> Result<RawHtml<String>, Status> {
    match get_entry(&entry) {
        Err(status) => Err(status),
        Ok(post) => {
            return Ok(RawHtml(
                Page {
                    content: post.content(),
                    title: post.title(),
                    category: Category::Blog,
                    description: post.description().into(),
                    keywords: join(post.tags().iter().map(|tag| tag.display_as()), ", "),
                    meow: Meow::from_blog(&post).ok(),
                    canonical: format!("{}/log/{}", CONFIG.base_url, post.slug()),
                    show_tags: false,
                    has_code: post.has_code(),
                }
                .render()
                .into_string(),
            ))
        }
    }
}
