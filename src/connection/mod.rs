//! Holds connection related data types.

mod closereason;
mod r#type;

pub use self::closereason::*;
pub use self::r#type::*;

use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{lookup_host, TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;

use crate::command::Command;
use crate::message::Message;
use crate::pushmessage::*;
use crate::reply::*;
use crate::word::Word;

use futures::channel::oneshot;
use futures::{channel::mpsc, Future};
use futures_util::sink::SinkExt;

struct ConnectionInternal {
    tag_counter: usize,
    reply_map: HashMap<Box<Word>, oneshot::Sender<Result<Reply, CloseReason>>>,
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
            InternalReply::HistoryInit(count) => {
                if count == 0 {
                    if let Some(sender) = self.reply_map.remove(&reply.0) {
                        sender.send(Ok(Reply::History(vec![]))).unwrap();
                    }
                } else {
                    self.awaiting_history = Some((count, Vec::with_capacity(count as usize)));
                }
            }
            InternalReply::HistoryMessage(index, message) => {
                match &self.awaiting_history {
                    None => panic!("not waiting"),
                    Some((count, items)) => {
                        let mut items = items.clone(); // REVIEW
                        items.push(message);
                        if index == *count - 1 {
                            // done
                            self.awaiting_history = None;

                            if let Some(sender) = self.reply_map.remove(&reply.0) {
                                sender.send(Ok(Reply::History(items))).unwrap();
                            }
                        } else {
                            self.awaiting_history = Some((*count, items));
                        }
                    }
                }
            }
            InternalReply::Normal(n) => {
                if let Some(sender) = self.reply_map.remove(&reply.0) {
                    sender.send(Ok(n)).unwrap();
                }
            }
        }
    }
}

/// A connection with a tomsg server.
pub struct Connection {
    stream: Arc<Mutex<OwnedWriteHalf>>,
    internal: Arc<Mutex<ConnectionInternal>>,
}

impl Connection {
    /// Creates a new `Connection` with the given `_typ` and connects to the given `address`.
    ///
    /// Returns a `Result` containing either an `io::Error` as an `Error` value, or a pair of a
    /// `Connection` and the receiver end of a `mpsc` channel where `PushMessage` instances are
    /// sent to.
    pub async fn connect(
        _typ: Type,
        address: impl ToSocketAddrs,
    ) -> std::io::Result<(Self, mpsc::Receiver<PushMessage>)> {
        let address = lookup_host(address)
            .await?
            .next()
            .ok_or(io::ErrorKind::AddrNotAvailable)?;

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

        let conn = Self {
            stream: Arc::new(Mutex::new(writer)),

            internal: internal.clone(),
        };

        tokio::spawn(async move {
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

        let version: &Word = "4".try_into().unwrap();
        conn.send_command(Command::Version(version.into()))
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

    /// Send the given `command` to this `Connection`.
    pub fn send_command<'a, 'b>(
        &'a self,
        command: Command<'b>,
    ) -> impl Future<Output = tokio::io::Result<Result<Reply, CloseReason>>> + 'a {
        let command = command.to_string();

        async move {
            let (tag, receiver) = {
                let mut internal = self.internal.lock().await;

                let tag = internal.tag_counter;
                let tag: Box<Word> = tag.to_string().try_into().unwrap();
                internal.tag_counter = internal.tag_counter.overflowing_add(1).0;

                let (sender, receiver) = oneshot::channel();
                if internal.reply_map.insert(tag.clone(), sender).is_some() {
                    // this shouldn't be possible.
                    panic!("key already exists");
                }
                (tag, receiver)
            };

            {
                let mut stream = self.stream.lock().await;
                stream
                    .write(format!("{} {}\n", tag, command).as_bytes())
                    .await?;
                stream.flush().await?;
            }

            Ok(receiver.await.unwrap())
        }
    }

    /// Gets the reason this `Connection` is closed, or `None` if the `Connection` is still open.
    pub async fn close_reason(&self) -> Option<CloseReason> {
        self.internal.lock().await.close_reason.clone()
    }
    /// Returns whether or not this `Connection` is closed.
    pub async fn is_closed(&self) -> bool {
        self.internal.lock().await.close_reason.is_some()
    }
}
