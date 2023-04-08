use crate::{
    assets::Asset,
    blinkies::Blinkybox,
    components::table::{Row, Table},
    markdown::Markdown,
    page::{Category, Page},
};
use maud::{html, Markup, Render};
use rocket::response::content::RawHtml;

lazy_static! {
    static ref HOME_PAGE: RawHtml<String> = get_home_page();
}

fn get_home_page() -> RawHtml<String> {
    let content = html! {
        p {
            ("hi! i'm zoe,  welcome to my website! ")
            a href="/index.xml"
            {
                ("an atom feed is available here.")
            }
        }
        // (Markdown(include_str!("aboutme.md")))
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
        title: "home",
        ..Default::default()
    };
    RawHtml(page.render().into_string())
}

#[get("/")]
pub fn home_page() -> RawHtml<String> {
    HOME_PAGE.to_owned()
}

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
