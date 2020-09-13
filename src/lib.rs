pub mod connection;

mod command;
mod id;
mod line;
mod message;
mod pushmessage;
mod reply;
mod util;
mod word;

// rexport
pub use connection::Connection;

// structs
pub use id::Id;
pub use line::Line;
pub use message::Message;
pub use word::Word;

// enums
pub use command::Command;
pub use pushmessage::PushMessage;
pub use reply::Reply;

/*
#[cfg(test)]
mod tests {
    use tokio::net::*;
    use tokio::task;

    use super::command::Command;
    use super::connection::*;

    use futures::stream::StreamExt;

    #[test]
    fn it_works() {
        task::block_on(async {
            let (mut conn, mut push_channel) = Connection::connect(
                Type::Plain,
                SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 29536).into(),
            )
            .await
            .unwrap();

            let t = task::spawn(async move {
                loop {
                    let push_message = push_channel.next().await;
                    println!("{:?}", push_message);
                }
            });

            let res = conn
                .sendMessage(Command::Register("ham4", "kaas"))
                .await
                .unwrap();
            println!("{:?}", res);
            let res = conn
                .sendMessage(Command::Login("ham4", "kaas"))
                .await
                .unwrap();
            println!("{:?}", res);
            let room_id = conn.sendMessage(Command::CreateRoom).await.unwrap();
            println!("room {:?}", room_id);

            t.await;
        })
    }
}
*/
