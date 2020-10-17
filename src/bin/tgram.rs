// use futures::StreamExt;
// use telegram_bot::*;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let token = String::from("1174014963:AAGCE4nJb0U9zX1rS2kBBK2IFeqUKaaKgjM");
//     let api = Api::new(token);
//     let mut stream = api.stream();
//     println!("Server established to accept connections.");
//     while let Some(update) = stream.next().await {
//         let update = update?;
//         if let UpdateKind::Message(message) = update.kind {
//             if let MessageKind::Text { ref data, .. } = message.kind {
//                 let y = format!("<{}>: {}", &message.from.first_name, data);

//                 println!("{}", y);
//                 api.send(message.text_reply(y.as_str())).await?;
//             }
//         }
//     }
//     Ok(())
// }

fn main() {
    todo!()
}
