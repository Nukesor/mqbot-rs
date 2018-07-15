use diesel;
use diesel::prelude::*;
use db::schema::entries;
use diesel::pg::PgConnection;

use db::models::playlist::Playlist;

#[derive(Queryable, Associations)]
#[table_name = "entries"]
#[belongs_to(Playlist, foreign_key="playlist_name")]
pub struct Entry {
    pub id: i64,
    pub playlist_name: String,
    pub url: String,
}

impl Entry {
    pub fn new(playlist_name: String, url: String, connection: &PgConnection) {
        let new_entry = NewEntry {
            url: url,
            playlist_name: playlist_name,
        };

        // Insert it into the database
        diesel::insert_into(entries::table)
            .values(&new_entry)
            .execute(connection)
            .expect("Error inserting new entry into database.");
    }
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub url: String,
    pub playlist_name: String,
}
