use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::BufReader;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::Mutex;
use tokio::task;

use super::closereason::CloseReason;
use super::r#type::Type;
use crate::command::Command;
use crate::message::Message;
use crate::pushmessage::*;
use crate::reply::*;
use crate::word::Word;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures_util::sink::SinkExt;

struct ConnectionInternal {
    tag_counter: usize,
    reply_map: HashMap<Word, oneshot::Sender<Result<Reply, CloseReason>>>,
    awaiting_history: Option<(i64, Vec<Message>)>,
    push_channel: mpsc::Sender<PushMessage>,
    close_reason: Option<CloseReason>,
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
                if count == 0 {
                    if let Some(sender) = self.reply_map.remove(&reply.0) {
                        sender
                            .send(Ok(Reply {
                                tag: reply.0,
                                command: ReplyCommand::History(vec![]),
                            }))
                            .unwrap();
                    }
                } else {
                    self.awaiting_history = Some((count, Vec::with_capacity(count as usize)));
                }
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
                                    .send(Ok(Reply {
                                        tag: reply.0,
                                        command: ReplyCommand::History(items),
                                    }))
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
                        .send(Ok(Reply {
                            tag: reply.0,
                            command: n,
                        }))
                        .unwrap();
                }
            }
        }
    }
}

pub struct Connection {
    stream: OwnedWriteHalf,
    internal: Arc<Mutex<ConnectionInternal>>,
}

impl Connection {
    pub async fn connect(
        _typ: Type,
        address: SocketAddr,
    ) -> std::io::Result<(Self, mpsc::Receiver<PushMessage>)> {
        let stream = TcpStream::connect(address).await?;
        let (reader, writer) = stream.into_split();

        let (push_send, push_receive) = mpsc::channel(20);

        let internal = Arc::new(Mutex::new(ConnectionInternal {
            tag_counter: 0,
            reply_map: HashMap::new(),
            awaiting_history: None,
            push_channel: push_send,
            close_reason: None,
        }));

        let mut conn = Self {
            stream: writer,

            internal: internal.clone(),
        };

        task::spawn(async move {
            let mut reader = BufReader::new(reader);

            let (mut internal, close_reason) = loop {
                let mut line = String::new();
                let res = reader.read_line(&mut line).await;

                let mut internal = internal.lock().await;
                match res {
                    Err(e) => {
                        internal.push_channel.close_channel();

                        let close_reason = CloseReason::Err(e.to_string());
                        internal.close_reason = Some(close_reason.clone());
                        break (internal, close_reason);
                    }
                    Ok(0) => {
                        // EOF
                        internal.push_channel.close_channel();

                        let close_reason = CloseReason::EOF;
                        internal.close_reason = Some(close_reason.clone());
                        break (internal, close_reason);
                    }
                    Ok(_) => {
                        line.pop();
                        internal.handle_message(line).await;
                    }
                }
            };

            for (_, ch) in internal.reply_map.drain() {
                ch.send(Err(close_reason.clone())).unwrap();
            }
        });

        conn.send_message(Command::Version(Word::try_from("2".to_string()).unwrap()))
            .await?
            .map_err(|e| {
                let (kind, e) = match e {
                    CloseReason::EOF => (io::ErrorKind::ConnectionAborted, "EOF".to_owned()),
                    CloseReason::Err(e) => (io::ErrorKind::ConnectionReset, e),
                };
                io::Error::new(kind, e)
            })?;

        Ok((conn, push_receive))
    }

    async fn send_message_with_tag(
        &mut self,
        tag: Word,
        command: Command,
    ) -> std::io::Result<Result<Reply, CloseReason>> {
        let receiver = {
            let mut internal = self.internal.lock().await;

            if internal.reply_map.contains_key(&tag) {
                panic!("key already exists");
            }

            let (sender, receiver) = oneshot::channel();
            internal.reply_map.insert(tag.clone(), sender);
            receiver
        };

        self.stream
            .write(format!("{} {}\n", tag, command.to_str()).as_bytes())
            .await?;
        self.stream.flush().await?;

        let value = receiver.await.unwrap();
        Ok(value)
    }

    pub async fn send_message(
        &mut self,
        command: Command,
    ) -> tokio::io::Result<Result<Reply, CloseReason>> {
        let tag = {
            let mut internal = self.internal.lock().await;

            let tag = internal.tag_counter;
            internal.tag_counter += 1;
            tag
        };

        self.send_message_with_tag(Word::try_from(tag.to_string()).unwrap(), command)
            .await
    }

    pub async fn close_reason(&self) -> Option<CloseReason> {
        self.internal.lock().await.close_reason.clone()
    }
    pub async fn is_closed(&self) -> bool {
        self.internal.lock().await.close_reason.is_some()
    }
}
