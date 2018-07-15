use db::schema::users_playlists;
use db::schema::users_playlists::dsl as up_dsl;
use db::models::user::User;
use db::models::playlist::Playlist;

#[derive(Queryable, Insertable, Associations)]
#[table_name = "users_playlists"]
#[belongs_to(User)]
#[belongs_to(Playlist, foreign_key="playlist_name")]
pub struct UserPlaylist {
    pub id: i64,
    pub user_name: String,
    pub playlist_name: String,
    pub last_entry_id: i64,
}


impl UserPlaylist {
    pub fn get_next(playlist_name: &str,
                    user_name: &str,
                    connection: &PgConnection) -> Result<Playlist, Error> {

        let user_playlist = up_dsl::users_playlists
            .filter(up_dsl::playlist_name.eq(playlist_name.to_string()))
            .filter(up_dsl::user_name.eq(playlist_name.to_string()))
            .get_result::<Playlist>(connection)

        entries::dsl::entries
            .filter(entries::dsl::plylist_name.eq(playlist_name.to_string()))
            .get_result::<Playlist>(connection)
    }

}
