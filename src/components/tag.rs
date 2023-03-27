use maud::{html, Markup, Render};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, PartialEq, Clone)]
pub enum Tag {
    Animals,
    Doggies,
    Kitties,
    Meow,
}

impl Tag {
    pub fn link(&self) -> Vec<&'static str> {
        vec![
            "/",
            "blog/",
            "tags/",
            match &self {
                Tag::Animals => "animals/",
                Tag::Doggies => "doggies/",
                Tag::Kitties => "kitties/",
                Tag::Meow => "meow/",
            },
        ]
    }
    pub fn display_as(&self) -> &'static str {
        match &self {
            Tag::Animals => "animals",
            Tag::Doggies => "doggies",
            Tag::Kitties => "kitties",
            Tag::Meow => "meow",
        }
    }
}

impl Render for Tag {
    fn render(&self) -> Markup {
        html! {
        a
        class="tag"
        href=(self.link().concat())
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
