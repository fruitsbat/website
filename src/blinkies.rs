use maud::{html, Render};
use strum::{EnumIter, IntoEnumIterator};

use crate::files::{Asset, AssetType};

#[derive(EnumIter)]
pub enum Blinkies {
    Trains,
    Abductable,
    Frickinbats,
}

impl Blinkies {
    pub fn asset(&self) -> Asset {
        let asset_type = AssetType::Image;
        match &self {
            Self::Trains => Asset {
                path: vec!["blinkies/", "trains.gif"],
                content: include_bytes!("assets/blinkies/trains.gif"),
                asset_type,
                alt: "a blinky that says this user is trains",
            },
            Self::Abductable => Asset {
                path: vec!["blinkies/", "abductable.gif"],
                content: include_bytes!("assets/blinkies/abductable.gif"),
                asset_type,
                alt: "an image of an alien that says abductable next to it",
            },
            Self::Frickinbats => Asset {
                path: vec!["blinkies/", "frickinbats.gif"],
                content: include_bytes!("assets/blinkies/frickinbats.gif"),
                asset_type,
                alt: "a few bat shapes on a black background",
            },
        }
    }
}

impl Blinkies {
    pub fn url(&self) -> &'static str {
        match &self {
            Self::Trains => "https://web.archive.org/web/20220510005638/https://pronoun.is/fae",
            Self::Abductable => "https://memory-alpha.fandom.com/wiki/Baseball?so=search",
            Self::Frickinbats => "https://en.wikipedia.org/wiki/Indian_flying_fox#/media/File:Indian_flying_fox_cropped.jpg",
        }
    }
}

impl Render for Blinkies {
    fn render(&self) -> maud::Markup {
        html! {
            a href=(self.url()) {
                img alt=(self.asset().alt) src=(format!("/assets/images/{}", self.asset().path.concat())) {}
            }
        }
    }
}

pub struct Blinkybox;
impl Render for Blinkybox {
    fn render(&self) -> maud::Markup {
        html! {
            div class="blinkybox" {
                @for blinky in Blinkies::iter() {
                    (blinky)
                }
            }
        }
    }
}
