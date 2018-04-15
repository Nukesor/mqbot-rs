table! {
    users (id) {
        id -> BigInt,
        name -> Text,
    }
}

table! {
    playlists (id) {
        id -> BigInt,
        name -> Text,
    }
}

table! {
    entries (id) {
        id -> BigInt,
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
        user_id -> BigInt,
        playlist_id -> BigInt,
        last_entry_id -> BigInt,
    }
}
