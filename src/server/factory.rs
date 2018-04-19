use rocket::{ignite, Rocket};

use server::api;

pub fn rocket_factory() -> Rocket {
    ignite()
        .mount("/get_next", routes![
               api::playlist::get_next,
        ])
}
