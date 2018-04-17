extern crate mediaq;
extern crate futures;
extern crate tokio_core;
extern crate telegram_bot;
#[macro_use] extern crate indoc;
#[macro_use] extern crate diesel_migrations;

use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

use mediaq::settings::get_settings;
use mediaq::db::establish_connection;
use mediaq::db::models::chat::Chat;
use mediaq::db::models::entry::Entry;
use mediaq::db::models::playlist::Playlist;

embed_migrations!();

/// Run migrations and start the telegram bot
fn main() {
    // Get a database connection;
    let connection = establish_connection();

    // Run all migrations and output the log to stdout.
    let result = embedded_migrations::run_with_output(
        &connection, &mut std::io::stdout());

    if result.is_err() {
        println!("Migration failed.");
        panic!("Error: {:?}", result);
    }

    start_mq_bot();
}

/// Set up the mediaq bot and start polling
pub fn start_mq_bot() {
    // Get the settings from the config file.
    let settings = get_settings();
    let token_result = settings.get("telegram_token");
    if token_result.is_none() {
        panic!("No setting for telegram_token in config.toml");
    }
    let token = token_result.unwrap();

    let mut core = Core::new().unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // Only look at new messages
        if let UpdateKind::Message(message) = update.kind {
            // Only consider text messages
            process_message(api.clone(), message);
        }

        Ok(())
    });

    core.run(future).unwrap();
}

/// Process any incoming messages from telegram with the type Text.
/// Check for each message if it is a command and if we support this command
fn process_message(api: Api, message: Message) {
    if let MessageKind::Text {ref data, ..} = message.kind {
        if !data.starts_with("/") {
            return;
        }

        let connection = establish_connection();

        if data.starts_with("/add") {
            let mut url = data.replace("/add", "");
            url = url.trim().to_string();
            let chat_id = message.chat.id().into();

            let result = Chat::get(chat_id, &connection);
            if result.is_err() {
                let response = "There is no playlist set for this chat yet. Use /setPlaylist";
                api.spawn(message.chat.text(response));

                return
            }
            let chat = result.unwrap();

            Entry::new(chat.playlist_name, url, &connection)
        }

        // Set the playlist for this chat.
        // We create a new one, in case the specified playlist doesn't exist.
        else if data.starts_with("/setPlaylist") {
            let mut playlist_name = data.replace("/setPlaylist", "");
            playlist_name = playlist_name.trim().to_string();

            // Create the playlist and the chat objects.
            Playlist::get_or_create(&playlist_name, &connection);
            let chat_id = message.chat.id().into();
            Chat::update_or_create(chat_id, &playlist_name, &connection);

            // Send the success response
            let response = format!("This chat now uses the playlist: {}", playlist_name);
            api.spawn(message.chat.text(response));
        }

        // Print a message with all infos.
        else if data.starts_with("/info") {
            let chat_id = message.chat.id().into();
            // Check if we already know this chat.
            // In case we don't, the user needs to specify a playlist.
            let result = Chat::get(chat_id, &connection);
            if result.is_err() {
                let response = "There is no playlist for this chat yet. Use /setPlaylist";
                api.spawn(message.chat.text(response));

                return
            }
            let chat = result.unwrap();

            // Send the info message.
            let response = format!("This chat ({}) uses the paylist: {}", chat_id, chat.playlist_name);
            api.spawn(message.chat.text(response));
        }

        // Send a help message with all available commands.
        else if data.starts_with("/help") {
            let response = indoc!("
            Hi there. This is the mqbot.
            These are the available commands:
                /add {url}
                    Add a new url to the playlist
                /setPlaylist {name}
                    Set the playlist name for this chat
                /info
                    Show the name of the current Playlist
                /help
                    Show this message.
            ");
            api.spawn(message.chat.text(response));
        }
    }
}
