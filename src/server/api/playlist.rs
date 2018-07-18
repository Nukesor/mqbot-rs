use actix::prelude::*;
use actix_web::*;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use diesel::result::Error as diesel_error;
use diesel::pg::PgConnection;
use futures::Future;

use db::models::user_playlist::UserPlaylist;
use db::models::entry::Entry;
use server::db::DbExecutor;
use server::factory::AppState;

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct NextEntry {
    pub playlist_name: String,
    pub user_name: String,
}

impl Message for NextEntry {
    type Result = Result<Vec<Entry>, diesel_error>;
}

impl Handler<NextEntry> for DbExecutor {
    type Result = Result<Vec<Entry>, diesel_error>;

    fn handle(&mut self, msg: NextEntry, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let query_result = UserPlaylist::get_next(msg.playlist_name, msg.user_name, conn);

        match query_result {
            Ok(entries) => Ok(entries),
            Err(error) => Err(error)
        }
    }
}

/// Next playlist entry handler
pub fn next_entries(
    (args, state): (Path<(String, String)>, State<AppState>),
    ) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(NextEntry {
            playlist_name: args.0.clone(),
            user_name: args.1.clone(),
        })
    .from_err()
        .and_then(|result| match result {
            Ok(entries) => Ok(HttpResponse::Ok().json(entries)),
            Err(error) => {
                let error_msg = format!("{}", error);
                Ok(HttpResponse::InternalServerError().json(error_msg))
            }
        })
    .responder()
}
