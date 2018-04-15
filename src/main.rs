extern crate config;
extern crate futures;
extern crate tokio_core;
extern crate telegram_bot;
#[macro_use] extern crate indoc;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

pub mod db;
pub mod mq_bot;
pub mod settings;

use mq_bot::start_mq_bot;
use db::establish_connection;

embed_migrations!();

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

    start_mq_bot();
}
