use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::prelude::*,
    path::Path,
};

use bytes::Bytes;

pub fn write_data<W: Writable + ?Sized>(pages: &Vec<Box<W>>) -> Result<(), Box<dyn Error>> {
    for page in pages {
        // setup
        let path_string = format!("{}/{}", crate::EXPORT_PATH, &page.path());
        let path = Path::new(&path_string);

        // create it if file doesnt exist
        if !path.exists() {
            // create dir
            fs::create_dir_all(path.parent().ok_or("no parent dir set")?)?;
            OpenOptions::new().create_new(true).write(true).open(path)?;
        }

        let mut file = OpenOptions::new().read(true).open(path)?;
        let filecontents = page.filecontents();

        let mut current_filecontents = [];
        file.read(&mut current_filecontents)?;

        // only complain abt write access if its actually needed
        if !(Bytes::copy_from_slice(&current_filecontents) == filecontents) {
            let mut file = OpenOptions::new().write(true).open(path)?;
            file.write_all(&filecontents)?;
        }
    }
    Ok(())
}

pub enum AssetType {
    Font,
    Image,
    Video,
}

impl AssetType {
    pub fn folder(&self) -> &'static str {
        match &self {
            AssetType::Font => "fonts",
            AssetType::Image => "images",
            AssetType::Video => "videos",
        }
    }
}

pub struct Asset {
    pub path: Vec<&'static str>,
    pub content: &'static [u8],
    pub asset_type: AssetType,
}

impl Writable for Asset {
    fn path(&self) -> String {
        format!(
            "assets/{}/{}",
            self.asset_type.folder(),
            &self.path.concat()
        )
    }
    fn filecontents(&self) -> Bytes {
        Bytes::from_static(self.content)
    }
}

/// represents something that can be written to a file
pub trait Writable {
    fn path(&self) -> String;
    fn filecontents(&self) -> Bytes;
}
