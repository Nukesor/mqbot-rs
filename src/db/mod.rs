pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use settings::get_settings;

/// Create a new PostgreSQL connection to the database.
pub fn establish_connection() -> PgConnection {
    let settings = get_settings();
    let url = settings.get("database_url")
        .expect("No setting for database_url in config.toml");

    PgConnection::establish(&url)
        .expect(&format!("Error connecting to {}", url))
}
