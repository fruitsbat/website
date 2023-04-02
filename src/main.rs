#![feature(decl_macro)]

use maud::{html, Markup, Render};
use rocket::response::content::RawHtml;
use std::error::Error;

pub mod blinkies;
pub mod blog;
pub mod components;
pub mod files;
pub mod font;
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
    rocket::build().mount(
        "/",
        routes![home::home_page, blog::main_page, style::css, blinkies::file, font::regular],
    )
}
