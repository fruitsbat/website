use diesel::prelude::*;

#[derive(Queryable)]
pub struct Meow {
    pub number: usize,
    pub url: String,
}
