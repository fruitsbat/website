#![feature(decl_macro)]

pub mod assets;
pub mod blinkies;
pub mod blog;
pub mod components;
pub mod font;
pub mod home;
pub mod models;
pub mod page;
pub mod style;
pub mod tags;

#[macro_use]
extern crate rocket;

const EXPORT_PATH: &str = "result";

#[launch]
fn launch() -> _ {
    rocket::build().mount(
        "/",
        routes![
            home::home_page,
            blog::main_page,
            style::css,
            assets::file,
            font::regular,
        ],
    )
}
