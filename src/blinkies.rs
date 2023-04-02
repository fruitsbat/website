use maud::{html, Render};
use rocket::{
    http::{ContentType, Status},
    serde::{Deserialize, Serialize},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Serialize, Deserialize)]
pub enum Blinkies {
    Trains,
    Abductable,
    Frickinbats,
}

#[get("/assets/blinkies/<file>")]
pub fn file(file: String) -> Result<(ContentType, &'static [u8]), Status> {
    for blinkie in Blinkies::iter() {
        if blinkie.filename() == file {
            return Ok((ContentType::GIF, blinkie.content()));
        }
    }
    Err(Status::NotFound)
}

impl Blinkies {
    pub fn filename(&self) -> &'static str {
        match &self {
            Self::Trains => "trains.gif",
            Self::Abductable => "abductable.gif",
            Self::Frickinbats => "frickinbats.gif",
        }
    }

    pub fn content(&self) -> &'static [u8] {
        match self {
            Self::Trains => include_bytes!("assets/blinkies/trains.gif"),
            Self::Abductable => include_bytes!("assets/blinkies/abductable.gif"),
            Self::Frickinbats => include_bytes!("assets/blinkies/frickinbats.gif"),
        }
    }

    pub fn alt(&self) -> &'static str {
        match &self {
            Self::Trains => "this user is trains on top of a trans flag",
            Self::Abductable => "an alien, next to it it says abductible",
            Self::Frickinbats => "outlines of bats on a black background",
        }
    }

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
                img alt=(self.alt()) src=(format!("/assets/blinkies/{}", self.filename())) {}
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
