pub mod server {
    // use std::fmt::Error;

    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    };

    use crate::collaboration::{self, Command, SetCursorPosition};

    pub struct Server {
        tx: UnboundedSender<Command>,
        rx: UnboundedReceiver<Command>,
    }

    impl Server {
        pub fn new() -> Self {
            let (tx, rx) = unbounded_channel::<Command>();

            Server { tx, rx }
        }

        pub fn get_sender(&self) -> UnboundedSender<Command> {
            self.tx.clone()
        }

        pub async fn start(self, sender: UnboundedSender<Command>) -> Result<(), String> {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:6666")
                .await
                .unwrap();

            let (socket, _) = listener.accept().await.unwrap();
            let (mut reader, mut writer) = socket.into_split();
            let mut rx = self.rx;
            let tx = self.tx;

            tokio::spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    match cmd {
                        Command::SetCursorPosition(set_cursor_position) => {
                            let msg = format!(
                                "{}:{}",
                                set_cursor_position.note_id, set_cursor_position.pos
                            );
                            let _ = writer.write_all(msg.as_bytes()).await;
                        }
                        Command::InsertString(_) => {}
                        Command::Info(str) => {
                            let _ = writer.write_all(str.as_bytes()).await;
                        }
                    }
                }
            });

            let mut read_buf = vec![0u8; 4096];

            loop {
                match reader.read(&mut read_buf).await {
                    Ok(0) => return Err("Connection closed".to_string()), // Connection closed
                    Ok(n) => {
                        // let response = format!("'{}' byte read.", n);
                        // let _ = tx.send(Command::Info(response));

                        let s = String::from_utf8_lossy(&read_buf[0..n]);
                        let trimmed = s.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        // println!("Received: {}", trimmed);

                        let parts: Vec<&str> = trimmed.split(':').collect();
                        if parts.len() != 2 {
                            eprintln!("Invalid format. Expected 'index:pos', got: '{}'", trimmed);
                            continue;
                        }

                        let message_index = match parts[0].parse::<usize>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Parse error for index: {}", e);
                                continue;
                            }
                        };

                        let pos = match parts[1].parse::<usize>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Parse error for pos: {}", e);
                                continue;
                            }
                        };
                        // println!("SENDİNG");
                        _ = sender.send(collaboration::Command::SetCursorPosition(
                            SetCursorPosition {
                                note_id: message_index,
                                pos,
                            },
                        ));
                    }
                    Err(e) => {
                        return Err(format!("Socket error: {}", e));
                    }
                }
            }
        }
    }
}

pub mod client {

    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    };

    use crate::collaboration::{self, Command, SetCursorPosition};

    pub struct Client {
        tx: UnboundedSender<Command>,
        rx: UnboundedReceiver<Command>,
    }

    impl Client {
        pub fn new() -> Self {
            let (tx, rx) = unbounded_channel::<Command>();

            Client { tx, rx }
        }

        pub fn get_sender(&self) -> UnboundedSender<Command> {
            self.tx.clone()
        }
        pub async fn start(self, app_sender: UnboundedSender<Command>) -> Result<(), String> {
            let stream = tokio::net::TcpStream::connect("127.0.0.1:6666")
                .await
                .unwrap();

            let (mut reader, mut writer) = stream.into_split();

            let mut rx = self.rx;
            let tx = self.tx;
            let _ = tokio::spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    match cmd {
                        Command::SetCursorPosition(set_cursor_position) => {
                            let msg = format!(
                                "{}:{}",
                                set_cursor_position.note_id, set_cursor_position.pos
                            );
                            let _ = writer.write_all(msg.as_bytes()).await;
                        }
                        Command::InsertString(_) => {}
                        Command::Info(str) => {
                            let _ = writer.write_all(str.as_bytes()).await;
                        }
                    }
                }
            });

            let mut read_buf = vec![0u8; 4096];

            loop {
                match reader.read(&mut read_buf).await {
                    Ok(0) => return Err("Connection closed".to_string()), // Connection closed
                    Ok(n) => {
                        // let response = format!("'{}' byte read.", n);
                        // let _ = tx.send(Command::Info(response));

                        let s = String::from_utf8_lossy(&read_buf[0..n]);
                        let trimmed = s.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        // println!("Received: {}", trimmed);

                        let parts: Vec<&str> = trimmed.split(':').collect();
                        if parts.len() != 2 {
                            eprintln!("Invalid format. Expected 'index:pos', got: '{}'", trimmed);
                            continue;
                        }

                        let message_index = match parts[0].parse::<usize>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Parse error for index: {}", e);
                                continue;
                            }
                        };

                        let pos = match parts[1].parse::<usize>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Parse error for pos: {}", e);
                                continue;
                            }
                        };
                        // println!("SENDİNG");
                        _ = app_sender.send(collaboration::Command::SetCursorPosition(
                            SetCursorPosition {
                                note_id: message_index,
                                pos,
                            },
                        ));
                    }
                    Err(e) => {
                        return Err(format!("Socket error: {}", e));
                    }
                }
            }
        }
    }
}
