-- Your SQL goes here
CREATE TABLE CHAT (
    id INT PRIMARY KEY,
    playlist_name TEXT NOT NULL,
    FOREIGN KEY (playlist_name) REFERENCES playlists(name)
)
