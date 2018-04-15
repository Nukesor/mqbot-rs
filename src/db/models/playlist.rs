use diesel;
use diesel::prelude::*;
use diesel::result::Error;

use db::schema::playlists;

#[derive(Queryable)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
}

impl Playlist {

    pub fn get(playlist_name: &str, connection: &SqliteConnection) -> Result<Playlist, Error> {
        playlists::dsl::playlists
            .filter(playlists::dsl::name.eq(playlist_name))
            .get_result::<Playlist>(connection)
    }

    pub fn get_or_create(playlist_name: &str, connection: &SqliteConnection) -> Playlist{
        let result = Playlist::get(playlist_name, connection);
        // A playlist with this name exists. Return the result.
        if result.is_ok() {
            return result.unwrap()
        }

        //There is no playlist with this name. Create a new one
        let new_playlist = NewPlaylist {
            name: playlist_name.to_string(),
        };

        // Insert it into the database
        diesel::insert_into(playlists::table)
            .values(&new_playlist)
            .execute(connection)
            .expect("Error inserting new playlist into database.");

        // As we are using SQLite we have to query again.
        Playlist::get(playlist_name, connection)
            .expect("No playlist despite just adding the playlist.")
    }
}



#[derive(Insertable)]
#[table_name = "playlists"]
pub struct NewPlaylist {
    pub name: String,
}


#[derive(Queryable)]
pub struct UsersPlaylists{
    pub id: i64,
    pub user_id: i64,
    pub playlist_id: i64,
    pub entry_id: i64,
}