pub mod schema;
pub mod models;

use diesel::prelude::*;
use settings::get_settings;

pub fn establish_connection() -> SqliteConnection {
    let settings = get_settings();
    let url = settings.get("database_url")
        .expect("No setting for database_url in config.toml");

    println!("Using database url: {}", url);
    SqliteConnection::establish(&url)
        .expect(&format!("Error connecting to {}", url))
}
