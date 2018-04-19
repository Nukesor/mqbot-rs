use db::schema::users_playlists;
use db::models::user::User;
use db::models::playlist::Playlist;

#[derive(Queryable, Insertable, Associations)]
#[table_name = "users_playlists"]
#[belongs_to(User)]
#[belongs_to(Playlist, foreign_key="playlist_name")]
pub struct UserPlaylist {
    pub id: i64,
    pub user_id: i64,
    pub playlist_name: String,
    pub last_entry_id: i64,
}
