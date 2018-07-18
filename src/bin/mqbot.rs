extern crate mediaq;
#[macro_use]
extern crate diesel_migrations;

use mediaq::bot::start_mq_bot;
use mediaq::db::establish_connection;

embed_migrations!();

/// Run migrations and start the telegram bot
fn main() {
    // Get a database connection;
    let connection = establish_connection();

    // Run all migrations and output the log to stdout.
    let result = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    if result.is_err() {
        println!("Migration failed.");
        panic!("Error: {:?}", result);
    }

    start_mq_bot();
}
