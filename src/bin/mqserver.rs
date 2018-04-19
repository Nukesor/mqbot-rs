extern crate mediaq;
#[macro_use] extern crate diesel_migrations;

use mediaq::db::establish_connection;
use mediaq::server::factory::rocket_factory;

embed_migrations!();

/// Run the mediaq server.
fn main() {
    // Get a database connection;
    let connection = establish_connection();

    // Run all migrations and output the log to stdout.
    let result = embedded_migrations::run_with_output(
        &connection, &mut std::io::stdout());

    if result.is_err() {
        println!("Migration failed.");
        panic!("Error: {:?}", result);
    }

    let server = rocket_factory();
    server.launch();
}
