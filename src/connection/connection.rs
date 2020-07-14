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
use crate::message::Message;
use crate::pushmessage::*;
use crate::reply::*;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures_util::sink::SinkExt;

struct ConnectionInternal {
    tag_counter: usize,
    reply_map: HashMap<String, oneshot::Sender<Reply>>,
    awaiting_history: Option<(i64, Vec<Message>)>,
    push_channel: mpsc::Sender<PushMessage>,
}

impl ConnectionInternal {
    async fn handle_message(&mut self, message: String) {
        if message.splitn(2, ' ').next() == Some("_push") {
            self.handle_push(message).await;
        } else {
            self.handle_reply(message).await;
        }
    }

    async fn handle_push(&mut self, message: String) {
        let push = match PushMessage::parse(&message) {
            Some(p) => p,
            None => return, // we can ignore this push
        };
        self.push_channel.send(push).await.unwrap();
    }

    async fn handle_reply(&mut self, message: String) {
        let reply = parse(&message);

        match reply.1 {
            InternalReplyCommand::HistoryInit(count) => {
                self.awaiting_history = Some((count, Vec::with_capacity(count as usize)));
            }
            InternalReplyCommand::HistoryMessage(index, message) => {
                match &self.awaiting_history {
                    None => panic!("not waiting"),
                    Some((count, items)) => {
                        let mut items = items.clone(); // REVIEW
                        items.push(message);
                        if index == *count - 1 {
                            // done
                            self.awaiting_history = None;

                            if let Some(sender) = self.reply_map.remove(&reply.0) {
                                sender
                                    .send(Reply {
                                        tag: reply.0,
                                        command: ReplyCommand::History(items),
                                    })
                                    .unwrap();
                            }
                        } else {
                            self.awaiting_history = Some((*count, items));
                        }
                    }
                }
            }
            InternalReplyCommand::Normal(n) => {
                if let Some(sender) = self.reply_map.remove(&reply.0) {
                    sender
                        .send(Reply {
                            tag: reply.0,
                            command: n,
                        })
                        .unwrap();
                }
            }
        }
    }
}

pub struct Connection {
    stream: TcpStream,
    internal: Arc<Mutex<ConnectionInternal>>,
}

impl Connection {
    pub async fn connect(
        _typ: Type,
        address: SocketAddr,
    ) -> async_std::io::Result<(Self, mpsc::Receiver<PushMessage>)> {
        let stream = TcpStream::connect(address).await?;
        let (push_send, push_receive) = mpsc::channel(20);

        let internal = Arc::new(Mutex::new(ConnectionInternal {
            tag_counter: 0,
            reply_map: HashMap::new(),
            awaiting_history: None,
            push_channel: push_send,
        }));

        let mut conn = Self {
            stream: stream.clone(),

            internal: internal.clone(),
        };

        task::spawn(async move {
            let mut reader = BufReader::new(stream);

            loop {
                let mut line = String::new();
                reader.read_line(&mut line).await.unwrap();
                line.pop();
                internal.lock().await.handle_message(line).await;
            }
        });

        conn.send_message(Command::Version("1")).await?;

        Ok((conn, push_receive))
    }

    async fn send_message_with_tag(
        &mut self,
        tag: String,
        command: Command<'_>,
    ) -> async_std::io::Result<Reply> {
        let receiver = {
            let mut internal = self.internal.lock().await;

            if internal.reply_map.contains_key(&tag) {
                panic!("key already exists");
            }

            let (sender, receiver) = oneshot::channel();
            internal.reply_map.insert(tag.clone(), sender);
            receiver
        };

        println!("sending {} {}", tag, command.to_str());
        self.stream
            .write(format!("{} {}\n", tag, command.to_str()).as_bytes())
            .await?;

        let value = receiver.await.unwrap();
        println!("{:?}", value);
        Ok(value)
    }

    pub async fn send_message(&mut self, command: Command<'_>) -> async_std::io::Result<Reply> {
        let tag = {
            let mut internal = self.internal.lock().await;

            let tag = internal.tag_counter;
            internal.tag_counter += 1;
            tag
        };

        self.send_message_with_tag(tag.to_string(), command).await
    }
}
