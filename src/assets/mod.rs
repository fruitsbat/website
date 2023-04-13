use cached::proc_macro::cached;
use rocket::http::{ContentType, Status};
use strum::{EnumIter, IntoEnumIterator};

// videos, images, sound, etc
#[derive(EnumIter, Copy, Clone)]
pub enum Asset {
    Me,
    FrickinbatsBlinkie,
    TrainsBlinkie,
    AbductableBlinkie,
    Kaokao,
    FirefoxUser,
    Silly,
    Blink,
}

impl Asset {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::TrainsBlinkie => "trains.gif",
            Self::AbductableBlinkie => "abductable.gif",
            Self::FrickinbatsBlinkie => "frickinbats.gif",
            Self::Me => "me.webp",
            Self::Kaokao => "kaokao.gif",
            Self::FirefoxUser => "firefoxuser.gif",
            Self::Silly => "getsilly.gif",
            Self::Blink => "blink.webp",
        }
    }

    pub fn content_type(&self) -> ContentType {
        match self {
            Self::Kaokao => ContentType::GIF,
            Self::FirefoxUser => ContentType::GIF,
            Self::Silly => ContentType::GIF,
            Self::Me => ContentType::JPEG,
            _ => ContentType::Any,
        }
    }

    pub fn content(&self) -> &'static [u8] {
        match self {
            Self::TrainsBlinkie => include_bytes!("blinkies/trains.gif"),
            Self::AbductableBlinkie => include_bytes!("blinkies/abductable.gif"),
            Self::FrickinbatsBlinkie => include_bytes!("blinkies/frickinbats.gif"),
            Self::Me => include_bytes!("images/me.webp"),
            Self::Kaokao => include_bytes!("images/kaokao.gif"),
            Self::FirefoxUser => include_bytes!("blinkies/firefoxuser.gif"),
            Self::Silly => include_bytes!("blinkies/getsilly.gif"),
            Self::Blink => include_bytes!("images/blink.webp"),
        }
    }

    pub fn alt(&self) -> &'static str {
        match self {
            Self::TrainsBlinkie => "this user is trains on top of a trans flag",
            Self::AbductableBlinkie => "an alien, next to it it says abductible",
            Self::Silly => "rainbow image that says 'get silly'",
            Self::FirefoxUser => "picture of a foxgirl, text saying firefox user next to that",
            Self::FrickinbatsBlinkie => "outlines of bats on a black background",
            Self::Me => "a picture of me",
            Self::Kaokao => {
                "a gif example showing off kaokao, an emoji is being selected from a big list"
            }
            Self::Blink => "the red led on a circuit playground blinking rythmically",
        }
    }
}

#[cached]
#[get("/assets/<file>")]
pub fn file(file: String) -> Result<(ContentType, &'static [u8]), Status> {
    for asset in Asset::iter() {
        if asset.filename() == file {
            return Ok((asset.content_type(), asset.content()));
        }
    }
    Err(Status::NotFound)
}
