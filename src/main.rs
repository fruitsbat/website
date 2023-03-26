pub mod components;
pub mod files;
pub mod page;
pub mod style;

const EXPORT_PATH: &'static str = "result";

fn main() {
    let index = page::Page {
        content: "meowowowowowow".into(),
        title: "home",
        category: page::Category::Home,
        path: vec!["/"],
    };
    let blog = page::Page {
        content: "eekejjkdfjsdkfjdsf".into(),
        title: "blog",
        category: page::Category::Blog,
        path: vec!["/", "blog"],
    };
    let v: Vec<Box<dyn files::Writable>> =
        vec![Box::new(index), Box::new(blog), Box::new(style::Css {})];
    match files::write_data(&v) {
        Err(e) => panic!("failed to write: {}", e),
        Ok(_) => (),
    };
}
