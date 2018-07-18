use actix::prelude::*;
use actix_web::*;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use futures::Future;

use db::models::user_playlist::UserPlaylist;
use server::db::DbExecutor;
use server::factory::AppState;

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct NextEntry {
    pub playlist_name: String,
    pub user_name: String,
}

impl Message for NextEntry {
    type Result = Result<String, Error>;
}

impl Handler<NextEntry> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: NextEntry, _: &mut Self::Context) -> Self::Result {
        Ok(msg.playlist_name)
    }
}

/// Next playlist entry handler
pub fn next_entry(
    (args, state): (Path<(String, String)>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(NextEntry {
            playlist_name: args.0.clone(),
            user_name: args.1.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
