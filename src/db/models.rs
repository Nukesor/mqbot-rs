use db::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String
}

#[derive(Queryable)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "playlists"]
pub struct NewPlaylist {
    pub name: String
}

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub url: String
}

#[derive(Queryable)]
pub struct UsersPlaylists{
    pub id: i32,
    pub user_id: i32,
    pub playlist_id: i32,
    pub entry_id: i32,
}
