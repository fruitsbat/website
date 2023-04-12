use crate::components::{footer::Footer, meow::Meow, tag::TagList};
use maud::{html, Markup, Render, DOCTYPE};
use strum::EnumIter;

#[derive(PartialEq, EnumIter, Copy, Clone)]
pub enum Category {
    Home,
    Blog,
}

impl Category {
    pub fn name(&self) -> &'static str {
        match &self {
            Category::Home => "home",
            Category::Blog => "weblog",
        }
    }
    pub fn link(&self) -> &'static str {
        match &self {
            Category::Home => "/",
            Category::Blog => "/log",
        }
    }
}

pub struct Page {
    pub content: Markup,
    pub category: Category,
    pub title: &'static str,
    pub show_tags: bool,
    pub meow: Option<Meow>,
    pub keywords: Option<String>,
    pub description: Option<String>,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            category: Category::Blog,
            title: "",
            show_tags: false,
            content: html! {},
            meow: None,
            keywords: None,
            description: None,
        }
    }
}

impl Render for Page {
    fn render(&self) -> Markup {
        let header = Header {
            title: self.title,
            show_tags: self.show_tags,
        };

        let meow = match &self.meow {
            None => html! {},
            Some(meow) => html! {(meow)},
        };

        let keywords = match &self.keywords {
            None => html! {},
            Some(keywords) => html! {
                meta
                    name="keywords"
                    content=(keywords){}
            },
        };

        let description = match &self.description {
            None => html! {},
            Some(description) => html! {
                meta
                    name="description"
                    content=(description) {}
            },
        };

        html! {
            (DOCTYPE)
            head {
                meta charset="UTF8" {}
                meta
                    name="viewport"
                    content="width=device-width"
                    initial-scale="1.0" {}
                title {(self.title)}
                link rel="stylesheet" href="/index.css" {}
                (keywords)
                (description)
                link
                    rel="alternate"
                    type="application/atom+xml"
                    href="/index.xml" {}
            }
            html lang=("en") {
                (header)
                body {
                    div #content {
                        (self.content.clone())
                    }
                    (meow)
                }
                (Footer {active: self.category})
            }
        }
    }
}

struct Header {
    title: &'static str,
    show_tags: bool,
}

impl Render for Header {
    fn render(&self) -> Markup {
        let tags = if self.show_tags {
            html! {(TagList {})}
        } else {
            html! {}
        };
        html! {
            Header {
                h1 {(self.title)}
                (tags)
            }
        }
    }
}
