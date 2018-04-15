table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    playlists (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    entries (id) {
        id -> Integer,
        url -> Text,
    }
}


table! {
    users_playlists (id) {
        id -> Integer,
        user_id -> Integer,
        playlist_id -> Integer,
        last_entry_id -> Integer,
    }
}
