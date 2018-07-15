use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

use db::schema::chats;
use db::models::playlist::Playlist;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Playlist, foreign_key="playlist_name")]
pub struct Chat {
    pub id: i64,
    pub playlist_name: String,
}


impl Chat {
    pub fn get(id: i64, connection: &PgConnection) -> Result<Chat, Error> {
        chats::dsl::chats
            .filter(chats::dsl::id.eq(id))
            .get_result::<Chat>(connection)
    }

    pub fn update_or_create(
        id: i64,
        playlist_name: &str,
        connection: &PgConnection) {
        // See if we already have a chat with this id.
        let result = chats::dsl::chats
            .filter(chats::dsl::id.eq(id))
            .get_result::<Chat>(connection);

        // A chat with this id already exists. Only update the playlist name.
        if result.is_ok() {
            diesel::update(chats::dsl::chats.find(id))
                .set(chats::dsl::playlist_name.eq(playlist_name))
                .execute(connection)
                .expect("Failed to update playlist name for chat.");

            return
        }

        //There is no chat with this id yet. Create a new one.
        let new_chat = NewChat {
            id: id,
            playlist_name: playlist_name.to_string(),
        };

        // Insert it into the database
        diesel::insert_into(chats::table)
            .values(&new_chat)
            .execute(connection)
            .expect("Error inserting new chat into database.");
    }
}

#[derive(Insertable)]
#[table_name = "chats"]
pub struct NewChat {
    pub id: i64,
    pub playlist_name: String,
}
