use actix::prelude::*;
use actix_web::{http, server, App};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

use server::api;
use server::db::DbExecutor;
use settings::get_settings;

/// State with DbExecutor address
pub struct AppState {
    pub db: Addr<Syn, DbExecutor>,
}

pub fn start_server() {
    let settings = get_settings();
    let url = settings
        .get("database_url")
        .expect("No setting for database_url in config.toml");

    // Start 3 db executor actors
    let manager = ConnectionManager::<PgConnection>::new(url.to_string());

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(10, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .resource("/entries/next/{playlist_name}/{user_name}", |r| {
                r.method(http::Method::GET).with(api::playlist::next_entries)
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();
}
