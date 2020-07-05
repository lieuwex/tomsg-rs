use std::collections::HashMap;

use async_std::io::BufReader;
use async_std::net::SocketAddr;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::Arc;
use async_std::sync::Mutex;
use async_std::task;

use super::r#type::Type;
use crate::command::Command;
use crate::pushmessage::*;
use crate::types::message::Message;
use crate::types::reply::*;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures_util::sink::SinkExt;

struct ConnectionInternal {
    tagCounter: usize,
    replyMap: HashMap<String, oneshot::Sender<Reply>>,
    awaitingHistory: Option<(i64, Vec<Message>)>,
    pushChannel: mpsc::Sender<PushMessage>,
}

impl ConnectionInternal {
    async fn handleMessage(&mut self, message: String) {
        if message.splitn(2, ' ').next() == Some("_push") {
            self.handlePush(message).await;
        } else {
            self.handleReply(message).await;
        }
    }

    async fn handlePush(&mut self, message: String) {
        println!("handlePush: {}", message);
        let push = match PushMessage::parse(&message) {
            Some(p) => p,
            None => return,
        };
        self.pushChannel.send(push).await;
    }

    async fn handleReply(&mut self, message: String) {
        println!("handleReply: {}", message);
        let reply = Reply::parse(&message);
        println!("we got a reply: {:?}", reply);

        match reply.command {
            ReplyCommand::historyInternal(count) => {
                self.awaitingHistory = Some((count, Vec::with_capacity(count as usize)));
            }
            ReplyCommand::historyMessageInternal(index, message) => {
                match &self.awaitingHistory {
                    None => panic!("not waiting"),
                    Some((count, items)) => {
                        let mut items = items.clone(); // REVIEW
                        items.push(message);
                        if index == *count - 1 {
                            // done
                            self.awaitingHistory = None;

                            if let Some(sender) = self.replyMap.remove(&reply.tag) {
                                sender
                                    .send(Reply {
                                        tag: reply.tag,
                                        command: ReplyCommand::History(items),
                                    })
                                    .unwrap();
                            }
                        } else {
                            self.awaitingHistory = Some((*count, items));
                        }
                    }
                }
            }
            _ => {
                if let Some(sender) = self.replyMap.remove(&reply.tag) {
                    println!("found sender for reply {}", reply.tag);
                    sender.send(reply).unwrap();
                }
            }
        }
    }
}

pub struct Connection {
    typ: Type,
    stream: TcpStream,
    internal: Arc<Mutex<ConnectionInternal>>,
}

impl Connection {
    pub async fn connect(
        typ: Type,
        address: SocketAddr,
    ) -> async_std::io::Result<(Self, mpsc::Receiver<PushMessage>)> {
        let stream = TcpStream::connect(address).await?;
        let (pushSend, pushReceive) = mpsc::channel(20);

        let internal = Arc::new(Mutex::new(ConnectionInternal {
            tagCounter: 0,
            replyMap: HashMap::new(),
            awaitingHistory: None,
            pushChannel: pushSend,
        }));

        let mut conn = Self {
            typ: typ,
            stream: stream.clone(),

            internal: internal.clone(),
        };

        task::spawn(async move {
            let mut reader = BufReader::new(stream);

            loop {
                let mut line = String::new();
                reader.read_line(&mut line).await.unwrap();
                line.pop();
                eprintln!("got: {}", line);
                internal.lock().await.handleMessage(line).await;
            }
        });

        println!("1");
        let p = conn.sendMessage(Command::Version("1")).await;
        println!("2");

        Ok((conn, pushReceive))
    }

    async fn sendMessageWithTag(
        &mut self,
        tag: String,
        command: Command<'_>,
    ) -> async_std::io::Result<Reply> {
        let receiver = {
            let mut internal = self.internal.lock().await;

            if internal.replyMap.contains_key(&tag) {
                panic!("key already exists");
            }

            let (sender, receiver) = oneshot::channel();
            internal.replyMap.insert(tag.clone(), sender);
            receiver
        };

        write!(self.stream, "{} {}\n", tag, command.to_str()).await?;
        eprintln!("written: {} {}", tag, command.to_str());

        Ok(receiver.await.unwrap())
    }

    pub async fn sendMessage(&mut self, command: Command<'_>) -> async_std::io::Result<Reply> {
        let tag = {
            let mut internal = self.internal.lock().await;

            let tag = internal.tagCounter;
            internal.tagCounter += 1;
            tag
        };

        self.sendMessageWithTag(tag.to_string(), command).await
    }
}

//Voorbeeld :
//
//
// let res = conn.sendMessageWithTag("1", Command::Version("5")).await;
//   // merk op dat uit ^ volgt dat sendMessageWithTag een future returnt
//
// res.id // dit zou dan "1" zijn
// res.args // dit zou dan de args zijn ofzo, moet even je protocol lezen
// res.typ // dit zou dan reply type zijn ofzo
