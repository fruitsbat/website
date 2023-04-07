pub mod assets;
pub mod blinkies;
pub mod blog;
pub mod components;
pub mod db;
pub mod feed;
pub mod font;
pub mod home;
mod migrations;
pub mod models;
pub mod page;
pub mod style;

#[macro_use]
extern crate rocket;

pub const URL: &'static str = "127.0.0.1:8000";

#[launch]
fn launch() -> _ {
    match db::run_migrations() {
        Err(e) => panic!("dead bc {}", e),
        Ok(_) => (),
    };
    rocket::build().mount(
        "/",
        routes![
            home::home_page,
            feed::feed,
            blog::main_page,
            blog::pages,
            style::css,
            assets::file,
            font::regular,
            components::tag::tags,
        ],
    )
}
