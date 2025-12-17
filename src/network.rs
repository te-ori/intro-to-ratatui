use crate::collaboration;

use tokio;

pub mod server {
    use tokio::io::AsyncReadExt;

    use crate::collaboration::{self, SetCursorPosition};

    pub async fn start(sender: tokio::sync::mpsc::UnboundedSender<collaboration::Command>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:6666")
            .await
            .unwrap();

        let (mut socket, _) = listener.accept().await.unwrap();
        let mut read_buf = vec![0u8; 4096];

        loop {
            match socket.read(&mut read_buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
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
                    if let Err(e) = sender.send(collaboration::Command::SetCursorPosition(
                        SetCursorPosition { message_index, pos },
                    )) {
                        eprintln!("Error sending command: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Socket error: {}", e);
                    break;
                }
            }
        }
    }
}
