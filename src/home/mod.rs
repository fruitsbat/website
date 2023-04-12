use crate::{
    assets::Asset,
    blinkies::Blinkybox,
    components::{
        table::{Row, Table},
        tag::Tag,
    },
    config::CONFIG,
    markdown::Markdown,
    page::{Category, Page},
};
use cached::proc_macro::cached;
use itertools::join;
use maud::{html, Markup, Render};
use rand::{seq::SliceRandom, thread_rng};
use rocket::response::content::RawHtml;
use strum::IntoEnumIterator;

#[get("/")]
pub fn home_page() -> RawHtml<String> {
    let recommended_tag: Tag = Tag::iter()
        .collect::<Vec<Tag>>()
        .choose(&mut thread_rng())
        .unwrap()
        .clone();
    let content = html! {
        p {
            "this is where i write about "
            a href=(format!("/tag/{}", recommended_tag.link())) {
                (recommended_tag.display_as())
            }
            ". "
            a href="/index.xml"
            {
                ("an atom feed is available here.")
            }
        }
        (aboutme())
        iframe
        frameBorder = "0"
        src="https://ring.bicompact.space/zoe-bat/pre"
        title="links from the armisael webring"
        {}
        (Blinkybox)
    };
    let page = Page {
        content,
        category: Category::Home,
        show_tags: false,
        meow: None,
        title: "hi! i'm zoe. welcome to my website",
        description: "my (zoe bat) personal blog, where i write about things".into(),
        keywords: join(Tag::iter().map(|t| t.display_as()), ", "),
        canonical: CONFIG.base_url.clone(),
    };
    RawHtml(page.render().into_string())
}

#[cached]
fn aboutme() -> Markup {
    let table = Table {
        rows: vec![
            Row {
                key: "name",
                value: html! {"zoe"},
            },
            Row {
                key: "pronouns",
                value: html! {"she/her, they/them, fae/faer"},
            },
            Row {
                key: "matrix",
                value: Markdown(
                    "[@bat:matrix.kittycat.homes](https://matrix.to/#/@bat:matrix.kittycat.homes)",
                )
                .render(),
            },
            Row {
                key: "gitea",
                value: Markdown("[zoe](https://git.kittycat.homes/zoe)").render(),
            },
            Row {
                key: "github",
                value: Markdown("[zoe-bat](https://github.com/zoe-bat)").render(),
            },
            Row {
                key: "bandcamp",
                value: Markdown("[zoebat](https://zoebat.bandcamp.com/)").render(),
            },
            Row {
                key: "itch",
                value: Markdown("[fruitsbat](https://fruitsbat.itch.io/)").render(),
            },
            Row {
                key: "loves",
                value: Markdown("[their girlfriend](https://tess.kittycat.homes)").render(),
            },
            Row {
                key: "trans",
                value: html! {"gender"},
            },
            Row {
                key: "non",
                value: html! {"binary"},
            },
            Row {
                key: "homo",
                value: html! {"sexual"},
            },
        ],
    };
    html! {
        p {
            h2 {"about me"}
            div class="aboutme" {
                img
                src=(format!("/assets/{}", Asset::Me.filename()))
                alt=(Asset::Me.alt())
                {}
                (table)
            }
        }
    }
}
