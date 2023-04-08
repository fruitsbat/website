use atom_syndication::{Category, Entry, Feed, Text};
use chrono::{DateTime, FixedOffset, TimeZone};
use rocket::response::content::RawXml;
use strum::IntoEnumIterator;

use crate::{blog::BlogEntry, components::tag::Tag, config::CONFIG};

/// servers a feed with every blog entry
#[get("/index.xml")]
pub fn feed() -> RawXml<String> {
    let feed = Feed {
        title: Text {
            value: String::from("zoe website"),
            ..Default::default()
        },
        lang: Some("en".into()),
        id: format!("{}/feed", CONFIG.base_url),
        updated: newest(),
        categories: Tag::iter().map(|f| f.category()).collect::<Vec<Category>>(),
        entries: BlogEntry::iter()
            .map(|entry| entry.entry())
            .collect::<Vec<Entry>>(),
        ..Default::default()
    };
    RawXml(feed.to_string())
}

/// determine when the feed was last updated
fn newest() -> DateTime<FixedOffset> {
    let mut largest = None;
    for e in BlogEntry::iter() {
        match largest {
            None => largest = Some(e.date()),
            Some(l) => {
                if l < e.date() {
                    largest = Some(e.date())
                }
            }
        };
    }

    // default to unix timestamp 0 to not crash
    let hour = 3600;
    largest.unwrap_or(
        FixedOffset::west_opt(2 * hour)
            .unwrap()
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap(),
    )
}
