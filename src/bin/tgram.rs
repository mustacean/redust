/*use futures::StreamExt;
use redust::service::Endpoint;
use redust::service::Service;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let user_service = Service::open("user_service", "127.0.0.1", &[], &[], &[]).unwrap();
    let sd = user_service.sender();

    use redusty::communication::IPost;

    let token = String::from("1174014963:AAGCE4nJb0U9zX1rS2kBBK2IFeqUKaaKgjM");
    let api = Api::new(token);
    let mut stream = api.stream();
    let ep_ = Endpoint::from_str("master/test");
    println!("Server established to accept connections.");
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                let y = format!("<{}>: {}", &message.from.first_name, data);

                let z = format!(
                    "{} ---> {:?}",
                    y.clone(),
                    ep_.post(&sd, serde_json::Value::String(y))
                );

                println!("{}", z);
                api.send(message.text_reply(z.as_str())).await?;
            }
        }
    }
    Ok(())
}
*/

fn main() {
    todo!()
}