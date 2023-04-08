use rocket::http::{ContentType, Status};
use strum::{EnumIter, IntoEnumIterator};

// videos, images, sound, etc
#[derive(EnumIter, Copy, Clone)]
pub enum Asset {
    Kittyroll,
    FrickinbatsBlinkie,
    TrainsBlinkie,
    AbductableBlinkie,
}

impl Asset {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::TrainsBlinkie => "trains.gif",
            Self::AbductableBlinkie => "abductable.gif",
            Self::FrickinbatsBlinkie => "frickinbats.gif",
            Self::Kittyroll => "funnycat.jpg",
        }
    }

    pub fn content_type(&self) -> ContentType {
        ContentType::Any
    }

    pub fn content(&self) -> &'static [u8] {
        match self {
            Self::TrainsBlinkie => include_bytes!("blinkies/trains.gif"),
            Self::AbductableBlinkie => include_bytes!("blinkies/abductable.gif"),
            Self::FrickinbatsBlinkie => include_bytes!("blinkies/frickinbats.gif"),
            Self::Kittyroll => include_bytes!("funnycat.jpg"),
        }
    }

    pub fn alt(&self) -> &'static str {
        match self {
            Self::TrainsBlinkie => "this user is trains on top of a trans flag",
            Self::AbductableBlinkie => "an alien, next to it it says abductible",
            Self::FrickinbatsBlinkie => "outlines of bats on a black background",
            Self::Kittyroll => "a cat all rolled up",
        }
    }
}

#[get("/assets/<file>")]
pub fn file(file: String) -> Result<(ContentType, &'static [u8]), Status> {
    for asset in Asset::iter() {
        if asset.filename() == file {
            return Ok((asset.content_type(), asset.content()));
        }
    }
    Err(Status::NotFound)
}
