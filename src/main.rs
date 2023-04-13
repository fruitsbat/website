use rocket::figment::Figment;

pub mod assets;
pub mod blinkies;
pub mod blog;
pub mod components;
pub mod config;
pub mod db;
pub mod feed;
pub mod font;
pub mod home;
pub mod markdown;
mod migrations;
pub mod models;
pub mod page;
mod robots;
pub mod schema;
mod sitemap;
pub mod style;
mod verification;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[launch]
fn launch() -> _ {
    if let Err(e) = db::run_migrations() {
        panic!("dead bc {}", e)
    }
    let routes = routes![
        home::home_page,
        feed::feed,
        blog::main_page,
        blog::pages,
        style::css,
        assets::file,
        font::regular,
        font::bold,
        font::mono,
        components::tag::tags,
        components::meow::meow,
        robots::robots,
        verification::google,
    ];
    rocket::custom(Figment::from(rocket::Config::default()).merge(("port", config::CONFIG.port)))
        .mount("/", routes)
}
