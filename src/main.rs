use maud::html;

pub mod blog;
pub mod components;
pub mod files;
pub mod page;
pub mod style;
pub mod tags;

const EXPORT_PATH: &'static str = "result";

fn main() {
    let index = page::Page {
        content: html! {},
        title: "home",
        category: page::Category::Home,
        path: vec!["/"],
    };
    let mut v: Vec<Box<dyn files::Writable>> = vec![
        Box::new(index),
        Box::new(style::Css),
        Box::new(files::Asset {
            path: vec!["atkinson.woff2"],
            asset_type: files::AssetType::Font,
            content: include_bytes!("font/atkinson_regular.woff2"),
            alt: "",
        }),
    ];
    v.append(&mut blog::blog_pages());
    match files::write_data(&v) {
        Err(e) => panic!("failed to write: {}", e),
        Ok(_) => (),
    };
}
