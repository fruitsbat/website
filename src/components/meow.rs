use crate::{blog::BlogEntry, db};
use diesel::prelude::*;
use maud::{html, Markup, Render};
use std::error::Error;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::meows)]
pub struct Meow {
    /// total number of times that people meowed at this page
    pub number: i64,
    /// slug of the blogpost this counter uses
    pub blog: String,
}

impl Meow {
    pub fn from_blog(blog: &BlogEntry) -> Result<Self, Box<dyn Error>> {
        use crate::schema::meows::dsl;
        let connection = &mut db::establish_connection()?;
        let result = dsl::meows
            .find(blog.slug())
            .get_result::<Meow>(connection)?;
        Ok(Meow {
            number: 0,
            blog: "meow".into(),
        })
    }
}

impl Render for Meow {
    fn render(&self) -> Markup {
        html! {}
    }
}
