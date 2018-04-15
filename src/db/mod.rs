pub mod schema;
pub mod models;

use diesel::prelude::*;
use settings::get_settings;

pub fn establish_connection() -> SqliteConnection {
    let settings = get_settings();
    let url_result = settings.get("database_url");
    if url_result.is_none() {
        panic!("No setting for database_url in config.toml");
    }
    let url = url_result.unwrap();

    println!("Using database url: {}", url);
    SqliteConnection::establish(&url)
        .expect(&format!("Error connecting to {}", url))
}
