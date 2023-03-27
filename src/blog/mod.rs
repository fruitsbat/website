use maud::{html, Markup};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    components::{
        linkbox::{Linkbox, LinkboxContainer},
        tag::Tag,
    },
    files::{Asset, AssetType, Writable},
    page::Page,
    tags::tags_page,
};
pub fn blog_pages() -> Vec<Box<dyn Writable>> {
    let mut writables: Vec<Box<dyn Writable>> = vec![];
    for post in BlogEntries::iter() {
        writables.append(&mut post.get().assets);
        match post.get().preview_image {
            None => (),
            Some(image) => writables.append(&mut vec![Box::new(image)]),
        }
        writables.append(&mut vec![Box::new(Page {
            content: post.get().content,
            category: crate::page::Category::Blog,
            path: vec!["/", "blog/", post.get().slug],
            title: post.get().title,
        })]);
    }
    for tag in Tag::iter() {
        let mut links = vec![];
        for page in BlogEntries::iter() {
            if page.get().tags.contains(&tag) {
                links.push(page.get().linkbox());
            }
        }
        writables.append(&mut vec![Box::new(Page {
            content: html! {(LinkboxContainer {linkboxes: links})},
            category: crate::page::Category::Blog,
            path: tag.link(),
            title: tag.display_as(),
        })]);
    }
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
    writables.append(&mut vec![Box::new(main_page), Box::new(tags_page())]);
    writables
}

pub struct BlogEntry {
    pub assets: Vec<Box<dyn Writable>>,
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
            image: self.preview_image.clone(),
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
                assets: vec![],
                preview_image: None,
                tags: vec![Tag::Doggies, Tag::Animals],
                content: html! {},
                slug: "doggies/",
                title: "doggies",
                description: "doggies know nothing they heads are empty",
            },
            BlogEntries::Kitties => BlogEntry {
                assets: vec![],
                preview_image: Some(Asset {
                    path: vec!["funnycat.jpg"],
                    content: include_bytes!("../assets/funnycat.jpg"),
                    asset_type: AssetType::Image,
                    alt: "a small kitty smiling and doing a :3 face",
                }),
                tags: vec![Tag::Kitties, Tag::Animals, Tag::Meow],
                content: html! {},
                slug: "kitties/",
                title: "kitties",
                description: "kittes know how to see a ghost (scary)",
            },
        }
    }
}
