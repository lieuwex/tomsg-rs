mod command;
mod connection;
mod pushmessage;
mod types;

pub fn test() {
    println!("test");
}

#[cfg(test)]
mod tests {
    use async_std::net::*;
    use async_std::task;

    use super::command::Command;
    use super::connection::*;

    use futures::stream::StreamExt;

    #[test]
    fn it_works() {
        task::block_on(async {
            println!("a");
            let (mut conn, mut pushChannel) = Connection::connect(
                Type::Plain,
                SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 29536).into(),
            )
            .await
            .unwrap();
            println!("b");

            let t = task::spawn(async move {
                loop {
                    let pushMessage = pushChannel.next().await;
                    println!("{:?}", pushMessage);
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
