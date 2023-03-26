use crate::page::Category;
use maud::{html, Markup, Render};
use strum::IntoEnumIterator;

pub struct Footer {
    pub active: Category,
}

impl Render for Footer {
    fn render(&self) -> Markup {
        html! {footer {
            @for category in Category::iter() {
                (Navlink  {active: self.active, category})
            }
        }}
    }
}

struct Navlink {
    // currently active category
    pub active: Category,

    // the links actual category
    pub category: Category,
}

impl Render for Navlink {
    fn render(&self) -> Markup {
        let class = if self.active == self.category {
            "active footer-button"
        } else {
            "inactive footer-button"
        };
        html! {
            a
            href=(self.category.link())
            class=(class)
            {(self.category.name())}
        }
    }
}
