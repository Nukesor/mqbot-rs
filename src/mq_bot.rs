use telegram_bot::*;
use diesel::prelude::*;
use tokio_core::reactor::Core;

use futures::Stream;
use settings::get_settings;

pub fn start_mq_bot(connection: &SqliteConnection) {
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

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
