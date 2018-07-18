use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use db::models::entry::Entry;
use db::models::playlist::Playlist;
use db::models::user::User;
use db::schema::entries;
use db::schema::users_playlists;
use db::schema::users_playlists::dsl as up_dsl;

#[derive(Queryable, Insertable, Associations)]
#[table_name = "users_playlists"]
#[belongs_to(User, foreign_key = "user_name")]
#[belongs_to(Playlist, foreign_key = "playlist_name")]
pub struct UserPlaylist {
    pub id: i64,
    pub user_name: String,
    pub playlist_name: String,
    pub last_entry_id: i64,
}

impl UserPlaylist {
    pub fn get_next(
        playlist_name: String,
        user_name: String,
        connection: &PgConnection,
    ) -> Result<Vec<Entry>, Error> {
        let user_playlist = up_dsl::users_playlists
            .filter(up_dsl::playlist_name.eq(playlist_name.to_string()))
            .filter(up_dsl::user_name.eq(playlist_name.to_string()))
            .get_result::<UserPlaylist>(connection)?;

        entries::dsl::entries
            .filter(entries::dsl::playlist_name.eq(playlist_name.to_string()))
            .filter(entries::dsl::id.ge(user_playlist.last_entry_id))
            .get_results::<Entry>(connection)
    }
}
