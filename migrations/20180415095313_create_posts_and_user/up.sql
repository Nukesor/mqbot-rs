-- Your SQL goes here
CREATE TABLE users (
    id INT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE playlists (
    id INT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE entries (
    id INT PRIMARY KEY,
    url TEXT UNIQUE NOT NULL
);

CREATE TABLE users_playlists (
    id INT PRIMARY KEY,
    user_id INT NOT NULL,
    playlist_id INT NOT NULL,
    last_entry_id INT NOT NULL DEFAULT 0,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(playlist_id) REFERENCES playlists(id)
);
