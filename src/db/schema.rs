table! {
    users (name) {
        name -> Text,
    }
}

table! {
    playlists (name) {
        name -> Text,
    }
}

table! {
    entries (id) {
        id -> BigInt,
        playlist_name -> Text,
        url -> Text,
    }
}

table! {
    chats (id) {
        id -> BigInt,
        playlist_name -> Text,
    }
}

table! {
    users_playlists (id) {
        id -> BigInt,
        user_name -> Text,
        playlist_name -> Text,
        last_entry_id -> BigInt,
    }
}
