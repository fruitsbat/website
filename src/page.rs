use crate::components::{footer::Footer, meow::Meow, tag::TagList};
use cached::proc_macro::cached;
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
    pub has_code: bool,
}

#[cached]
fn get_highlighting() -> Markup {
    html! {
        link
            rel="stylesheet"
            href="//unpkg.com/@catppuccin/highlightjs/css/catppuccin-frappe.css"
            {}
        script
            src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"
            {}
        script {"hljs.highlightAll();"}
    }
}

impl Render for Page {
    fn render(&self) -> Markup {
        let meow = match &self.meow {
            None => html! {},
            Some(meow) => html! {(meow)},
        };

        let header = Header {
            title: self.title,
            show_tags: self.show_tags,
            meow: meow.clone(),
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

        let highlighting = if self.has_code {
            get_highlighting()
        } else {
            html! {}
        };

        let canonical = html! {
            link
            rel="canonical"
            href=(self.canonical)
            {}
        };

        let head = html! {
            head {
                meta charset="utf-8" {}
                meta
                    name="viewport"
                    content="width=device-width"
                    initial-scale="1.0" {}
                title {(self.title)}
                link rel="stylesheet" href="/index.css" {}
                (keywords)
                (description)
                (highlighting)
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
                }
                (Footer {active: self.category})
            }
        }
    }
}

struct Header {
    title: &'static str,
    show_tags: bool,
    meow: Markup,
}

impl Render for Header {
    fn render(&self) -> Markup {
        let tags = if self.show_tags {
            html! {
                details {
                    summary {"show all tags"}
                    (TagList::default())
                }
            }
        } else {
            html! {}
        };
        html! {
            Header {
                div class="titletext" {
                    h1 {(self.title)}
                    (self.meow)
                }
                nav {(tags)}
            }
        }
    }
}
