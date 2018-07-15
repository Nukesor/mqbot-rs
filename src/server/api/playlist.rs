use db::establish_connection;
use db::models::user_playlist::UserPlaylist;
use server::responses::{APIResponse, ok};


#[get("/<playlist>/<user_name>")]
pub fn get_next(playlist: String, user_name: String) -> Result<APIResponse, APIResponse> {
    let connection = establish_connection();
    let entry = UserPlaylist::get_next(playlist, user_name, &connection);

    Ok(ok())
}
