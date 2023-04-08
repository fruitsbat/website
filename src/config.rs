use serde_derive::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = confy::load("website", "config").unwrap_or(Config::default());
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub base_url: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: "postgres://username:password@localhost/database".into(),
            base_url: "http://127.0.0.1:8000".into(),
            port: 8000,
        }
    }
}
