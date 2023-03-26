pub struct Css {}

impl crate::files::Writable for Css {
    fn filecontents(&self) -> String {
        grass::include!("src/css/index.scss").into()
    }
    fn path(&self) -> String {
        "index.css".into()
    }
}
