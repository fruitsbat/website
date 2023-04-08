use crate::config::CONFIG;
use diesel::{pg::PgConnection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    PgConnection::establish(&CONFIG.db_url)
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let connection = &mut establish_connection()?;
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
