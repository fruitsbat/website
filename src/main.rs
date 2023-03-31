use std::error::Error;

use maud::html;
use rocket::response::content::RawHtml;

pub mod blinkies;
pub mod blog;
pub mod components;
pub mod files;
pub mod home;
pub mod models;
pub mod page;
pub mod style;
pub mod tags;

#[macro_use]
extern crate rocket;

const EXPORT_PATH: &'static str = "result";

#[launch]
fn launch() -> _ {
    let mut v: Vec<Box<dyn files::Writable>> = vec![
        Box::new(style::Css),
        Box::new(files::Asset {
            path: vec!["atkinson.woff2"],
            asset_type: files::AssetType::Font,
            content: include_bytes!("font/atkinson_regular.woff2"),
            alt: "",
        }),
    ];
    v.append(&mut blog::blog_pages());
    v.append(&mut home::home_page());
    //  match files::write_data(&v) {
    //     Err(e) => panic!("failed to write: {}", e),
    //     Ok(_) => (),
    // };
    rocket::build().mount("/", routes![main_page])
}

#[get("/")]
fn main_page() -> String {
    "hehe".into()
}
