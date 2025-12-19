use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::collaboration::{Command, SetCursorPosition};
use crate::network::{client, server};
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Mode {
    Server,
    Client,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "server" => Ok(Mode::Server),
            "client" => Ok(Mode::Client),
            _ => Err(format!("'{}' is not a valid Mode", s)),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Server => write!(f, "server"),
            Mode::Client => write!(f, "client"),
        }
    }
}

pub struct Listener {
    server: Option<server::Server>,
    client: Option<client::Client>,
    messenger_rx: UnboundedReceiver<Vec<u8>>,
    app_sender: UnboundedSender<Command>,
    mode: Mode,
}

impl Listener {
    pub async fn start(mut self) {
        let (listener_tx, mut listener_rx) = unbounded_channel::<Vec<u8>>();
        let network_tx: UnboundedSender<Vec<u8>>;
        match self.mode {
            Mode::Server => {
                let server = self.server.unwrap();
                network_tx = server.get_sender();
                tokio::spawn(async move { server.start(listener_tx).await });
            }
            Mode::Client => {
                let client = self.client.unwrap();
                network_tx = client.get_sender();
                tokio::spawn(async move { client.start(listener_tx).await });
            }
        }

        loop {
            tokio::select! {
                message_from_network = listener_rx.recv() => {
                     if let Some(msg) = message_from_network {
                         let msg = String::from_utf8_lossy(&msg).to_string();

                        if msg.len() < 1 {
                            continue;
                        }

                        let parts: Vec<&str> = msg.trim().split(':').collect();
                        let command: Command = if parts.len() == 1 {
                            Command::Info(parts[0].to_string())
                        } else if parts[0] == "pos" {
                            let note_id = usize::from_str_radix(parts[1], 10);
                            if note_id.is_err() {
                             continue;
                            }

                            let pos = usize::from_str_radix(parts[2], 10);
                            if pos.is_err() {
                                continue;
                            }

                            let note_id = note_id.unwrap();
                            let pos = pos.unwrap();

                            Command::SetCursorPosition(SetCursorPosition {
                                note_id,
                                pos,
                            })
                        } else {
                            continue;
                        };

                        _ = self.app_sender.send(command);
                    }
                }

                command_from_app = self.messenger_rx.recv() =>  {
                    if let Some(msg) = command_from_app {
                        _ = network_tx.send(msg);
                    }
                }
            }
        }
    }
}

pub struct Messenger {
    server: Option<server::Server>,
    client: Option<client::Client>,
    listener_tx: Option<UnboundedSender<Vec<u8>>>,
    mode: Mode,
}

impl Messenger {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Server => Messenger {
                server: Some(server::Server::new()),
                client: None,
                listener_tx: None,
                mode,
            },
            Mode::Client => Messenger {
                server: None,
                client: Some(client::Client::new()),
                listener_tx: None,
                mode,
            },
        }
    }

    pub fn spawn_listener(&mut self, app_sender: UnboundedSender<Command>) -> Listener {
        let (listener_tx, listener_rx) = unbounded_channel::<Vec<u8>>();
        self.listener_tx = Some(listener_tx);
        Listener {
            server: self.server.take(),
            client: self.client.take(),
            messenger_rx: listener_rx,
            app_sender,
            mode: self.mode,
        }
    }

    pub fn send_cursor_position_command(&self, note_id: usize, pos: usize) {
        let msg = format!("pos:{}:{}", note_id, pos);
        match &self.listener_tx {
            Some(lsn) => {
                _ = lsn.send(msg.as_bytes().to_vec());
            }
            None => {}
        }
    }
}
