-- Your SQL goes here
CREATE TABLE chats (
    id BIGINT PRIMARY KEY,
    playlist_name TEXT NOT NULL,
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
)
