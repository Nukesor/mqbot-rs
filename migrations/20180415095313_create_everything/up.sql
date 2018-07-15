-- Your SQL goes here
CREATE TABLE users (
    name TEXT PRIMARY KEY
);

CREATE TABLE playlists (
    name TEXT PRIMARY KEY
);

-- Your SQL goes here
CREATE TABLE chats (
    id BIGINT PRIMARY KEY,
    playlist_name TEXT NOT NULL,
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
);

CREATE TABLE entries (
    id BIGSERIAL PRIMARY KEY,
    playlist_name TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
);

CREATE TABLE users_playlists (
    id BIGSERIAL PRIMARY KEY,
    user_name TEXT NOT NULL,
    playlist_name TEXT NOT NULL,
    last_entry_id BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (user_name) REFERENCES users(name),
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
);
