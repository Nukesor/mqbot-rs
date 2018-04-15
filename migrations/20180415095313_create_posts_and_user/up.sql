-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE playlists (
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE entries (
    id BIGSERIAL PRIMARY KEY,
    url TEXT UNIQUE NOT NULL
);

CREATE TABLE users_playlists (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    playlist_id BIGINT NOT NULL,
    last_entry_id BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (playlist_id) REFERENCES playlists(id)
);
