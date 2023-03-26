use bytes::Bytes;

pub struct Css;
impl crate::files::Writable for Css {
    fn filecontents(&self) -> Bytes {
        Bytes::copy_from_slice(grass::include!("src/css/index.scss").as_bytes())
    }
    fn path(&self) -> String {
        "index.css".into()
    }
}
