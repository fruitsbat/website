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
    pub keywords: String,
    pub description: String,
    pub canonical: String,
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

        let keywords = html! {
            meta
                name="keywords"
                content=(self.keywords){}
        };

        let description = html! {
                meta
                    name="description"
                    content=(self.description) {}
        };

        let canonical = html! {
            link
            rel="canonical"
            href=(self.canonical)
            {}
        };

        let head = html! {
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
                (canonical)
                link
                    rel="alternate"
                    type="application/atom+xml"
                    href="/index.xml" {}
            }
        };

        html! {
            (DOCTYPE)
            html lang=("en") {
                (head)
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
            html! {
                details {
                    summary {"show all tags"}
                    (TagList {})
                }
            }
        } else {
            html! {}
        };
        html! {
            Header {
                h1 {(self.title)}
                nav {(tags)}
            }
        }
    }
}
