use crate::{
    blog::{get_entry, BlogEntry},
    db,
};
use diesel::prelude::*;
use maud::{html, Markup, Render};
use rocket::{http::Status, response::Redirect};
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
        use crate::schema::meows::{self, dsl};
        let connection = &mut db::establish_connection()?;
        let result = dsl::meows.find(blog.slug()).get_result::<Meow>(connection);
        match result {
            Ok(result) => Ok(result),
            Err(_) => {
                // not found, so we insert a new one
                Ok(diesel::insert_into(meows::table)
                    .values(&Meow {
                        number: 0,
                        blog: blog.slug().into(),
                    })
                    .get_result::<Meow>(connection)?)
            }
        }
    }
}

impl Render for Meow {
    fn render(&self) -> Markup {
        html! {
            a href=(format!("/log/{}/meow", self.blog))
            class="meowbutton"
            {
                div class="meownumber" {(format!("{} ", self.number))}
                div class="meowbutton-text" {("🐈 meow at this page ")}
            }
        }
    }
}

#[get("/log/<entry>/meow")]
pub fn meow(entry: String) -> Result<Redirect, Status> {
    use crate::schema::meows::dsl;
    let blog = match get_entry(&entry) {
        Err(e) => return Err(e),
        Ok(b) => b,
    };

    // get old meow count
    let count = match Meow::from_blog(&blog) {
        Err(_) => return Err(Status::InternalServerError),
        Ok(m) => m.number.saturating_add(1),
    };

    let mut connection = match db::establish_connection() {
        Err(_) => return Err(Status::InternalServerError),
        Ok(c) => c,
    };

    match diesel::update(dsl::meows.find(blog.slug()))
        .set(dsl::number.eq(count))
        .get_result::<Meow>(&mut connection)
    {
        Err(_) => Err(Status::InternalServerError),
        Ok(_) => Ok(Redirect::to(format!("/log/{}", blog.slug()))),
    }
}
