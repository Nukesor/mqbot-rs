use rocket_contrib::{Json, SerdeError};

use db::establish_connection;
use server::responses::{APIResponse, bad_request, created, ok};


#[get("/<playlist>/<name>")]
pub fn get_next(playlist: String, name: String) -> Result<APIResponse, APIResponse> {
    let connection = establish_connection();

    Ok(ok())
}
