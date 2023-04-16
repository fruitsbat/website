use crate::{blog::BlogEntry, config::CONFIG};
use cached::proc_macro::cached;
use rocket::response::content::RawXml;
use sitewriter::UrlEntry;
use strum::IntoEnumIterator;

#[cached]
#[get("/sitemap.xml")]
pub fn sitemap() -> RawXml<String> {
    RawXml(sitewriter::generate_str(
        &vec![
            vec![
                UrlEntry {
                    loc: format!("{}", CONFIG.base_url).parse().unwrap(),
                    lastmod: None,
                    changefreq: None,
                    priority: Some(1.0),
                },
                UrlEntry {
                    loc: format!("{}/log", CONFIG.base_url).parse().unwrap(),
                    lastmod: None,
                    changefreq: None,
                    priority: Some(0.3),
                },
            ],
            BlogEntry::iter()
                .map(|b| b.url_entry())
                .collect::<Vec<UrlEntry>>(),
        ]
        .concat(),
    ))
}
