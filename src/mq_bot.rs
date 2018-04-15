use telegram_bot::*;
use tokio_core::reactor::Core;

use futures::Stream;
use settings::get_settings;

use db::establish_connection;
use db::models::playlist::Playlist;
use db::models::chat::Chat;

fn process_message(api: Api, message: Message) {
    if let MessageKind::Text {ref data, ..} = message.kind {
        if !data.starts_with("/") {
            return;
        }

        let connection = establish_connection();

        if data.starts_with("/add") {
            let mut url = data.replace("/add", "");
            url = url.trim().to_string();
        }
        else if data.starts_with("/setPlaylist") {
            let mut playlist_name = data.replace("/setPlaylist", "");
            playlist_name = playlist_name.trim().to_string();
            let playlist = Playlist::get_or_create(&playlist_name, &connection);

            let chat_id = message.chat.id().into();
            Chat::update_or_create(chat_id, &playlist_name, &connection);

            let response = format!("This chat now uses the playlist: {}", playlist_name);
            api.spawn(message.text_reply(response));
        }
        else if data.starts_with("/info") {
            let chat_id = message.chat.id().into();
            let result = Chat::get(chat_id, &connection);
            if result.is_err() {
                let response = "There is no playlist for this chat yet. Use /setPlaylist";
                api.spawn(message.text_reply(response));
            }
            let chat = result.unwrap();

            let response = format!("This chat uses the paylist: {}", chat.playlist_name);
            api.spawn(message.text_reply(response));
        }
        else if data.starts_with("/help") {
            let response = indoc!("
            Hi there. This is the mqbot.
            These are the available commands:
                /add {url}
                    Add a new url to the playlist
                /info
                    Show the name of the current Playlist
                /help
                    Show this message.
            ");
            api.spawn(message.text_reply(response));
        }
    }
}

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
