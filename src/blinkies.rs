use maud::{html, Render};
use strum::{EnumIter, IntoEnumIterator};

use crate::assets::Asset;

#[derive(EnumIter)]
pub enum Blinkies {
    Trains,
    Abductable,
    Frickinbats,
    Silly,
    FirefoxUser,
}

impl Blinkies {
    pub fn url(&self) -> &'static str {
        match &self {
            Self::Trains => "https://web.archive.org/web/20220510005638/https://pronoun.is/fae",
            Self::Abductable => "https://memory-alpha.fandom.com/wiki/Baseball?so=search",
            Self::Silly => "https://youtu.be/32Hp1LW08Yc",
            Self::FirefoxUser => "https://www.mozilla.org/en-US/firefox/new/",
            Self::Frickinbats => "https://en.wikipedia.org/wiki/Indian_flying_fox#/media/File:Indian_flying_fox_cropped.jpg",
        }
    }
    pub fn asset(&self) -> Asset {
        match self {
            Self::Trains => Asset::TrainsBlinkie,
            Self::Abductable => Asset::AbductableBlinkie,
            Self::Frickinbats => Asset::FrickinbatsBlinkie,
            Self::Silly => Asset::Silly,
            Self::FirefoxUser => Asset::FirefoxUser,
        }
    }
}

impl Render for Blinkies {
    fn render(&self) -> maud::Markup {
        html! {
            a href=(self.url()) {
                img alt=(self.asset().alt()) src=(format!("/assets/{}", self.asset().filename())) {}
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
