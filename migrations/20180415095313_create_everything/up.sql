-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
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
    user_id BIGINT NOT NULL,
    playlist_name TEXT NOT NULL,
    last_entry_id BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
);
