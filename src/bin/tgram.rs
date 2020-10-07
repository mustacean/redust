//use std::env;
use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = String::from("1174014963:AAGCE4nJb0U9zX1rS2kBBK2IFeqUKaaKgjM");
    //env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    println!("Server established to accept connections.");
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }
    Ok(())
}
