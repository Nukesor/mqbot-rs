# Mediaq-rs

This project is based on the [Mediaqbot](https://github.com/raffomania/mediaqbot) written by [Hans-Ole Hatzel](https://github.com/hatzel).


Its purpose is to receive media urls via a Telegram bot, store them in a database and serve the URLs via a http server to a client.
The client then downloads those urls with `youtube-dl` and plays them with `mpv`.
It is possible to manage the mqbot playlist via `mpv` as well as the telegram bot.


## Development

1. Create PostgreSQL database.
2. Copy `config.example.toml` to `config.toml` and adjust all settings.
3. Run: `cargo run`
