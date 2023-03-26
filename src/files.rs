use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::prelude::*,
    path::Path,
};

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
        let filecontents: String = page.filecontents();

        let mut current_filecontents = String::new();
        file.read_to_string(&mut current_filecontents)?;

        // only complain abt write access if its actually needed
        if !(current_filecontents == filecontents) {
            let mut file = OpenOptions::new().write(true).open(path)?;
            file.write_all(&filecontents.into_bytes())?;
        }
    }
    Ok(())
}

/// represents something that can be written to a file
pub trait Writable {
    fn path(&self) -> String;
    fn filecontents(&self) -> String;
}
