use crate::assets::Asset;

use super::tag::Tag;
use maud::{html, Markup, Render};

pub struct LinkboxContainer {
    pub linkboxes: Vec<Linkbox>,
}

impl Render for LinkboxContainer {
    fn render(&self) -> Markup {
        let boxes = html! {
            @for b in self.linkboxes.iter() {
                (b)
            }
        };
        html! {div class="linkbox-container" {(boxes)}}
    }
}

pub struct Linkbox {
    pub legend: String,
    /// link that this leads to
    pub path: String,
    pub image: Option<Asset>,
    pub description: String,
    pub tags: Vec<Tag>,
}

impl Render for Linkbox {
    fn render(&self) -> Markup {
        let image = match &self.image {
            None => html! {},
            Some(image) => html! {
                img src=(format!("/assets/{}",
                    image.filename()))
                    alt=(image.alt())
                 {}
            },
        };
        html! {
            fieldset role="presentation" class="linkbox" {
                legend {h2 {a href=(self.path) {(self.legend)}}}
                div {
                    p {(self.description)}
                    (image)
                    div class="tag-list" {
                        @for tag in self.tags.iter() {
                            (tag)
                        }
                    }
                }
            }
        }
    }
}
