pub mod server {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    };

    pub struct Server {
        tx: UnboundedSender<Vec<u8>>,
        rx: UnboundedReceiver<Vec<u8>>,
    }

    impl Server {
        pub fn new() -> Self {
            let (tx, rx) = unbounded_channel::<Vec<u8>>();

            Server { tx, rx }
        }

        pub fn get_sender(&self) -> UnboundedSender<Vec<u8>> {
            self.tx.clone()
        }

        pub async fn start(self, sender: UnboundedSender<Vec<u8>>) -> Result<(), String> {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:6666")
                .await
                .unwrap();

            let (socket, _) = listener.accept().await.unwrap();
            let (mut reader, mut writer) = socket.into_split();
            let mut rx = self.rx;

            tokio::spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    _ = writer.write(&cmd).await;
                }
            });

            let mut read_buf = vec![0u8; 4096];

            loop {
                match reader.read(&mut read_buf).await {
                    Ok(0) => return Err("Connection closed".to_string()), // Connection closed
                    Ok(n) => {
                        _ = sender.send(read_buf[0..n].to_vec());
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

    pub struct Client {
        tx: UnboundedSender<Vec<u8>>,
        rx: UnboundedReceiver<Vec<u8>>,
    }

    impl Client {
        pub fn new() -> Self {
            let (tx, rx) = unbounded_channel::<Vec<u8>>();

            Client { tx, rx }
        }

        pub fn get_sender(&self) -> UnboundedSender<Vec<u8>> {
            self.tx.clone()
        }
        pub async fn start(self, app_sender: UnboundedSender<Vec<u8>>) -> Result<(), String> {
            let stream = tokio::net::TcpStream::connect("127.0.0.1:6666")
                .await
                .unwrap();

            let (mut reader, mut writer) = stream.into_split();
            let mut rx = self.rx;

            tokio::spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    let _ = writer.write_all(&cmd).await;
                }
            });

            let mut read_buf = vec![0u8; 4096];

            loop {
                match reader.read(&mut read_buf).await {
                    Ok(0) => return Err("Connection closed".to_string()), // Connection closed
                    Ok(n) => {
                        _ = app_sender.send(read_buf[0..n].to_vec());
                    }
                    Err(e) => {
                        return Err(format!("Socket error: {}", e));
                    }
                }
            }
        }
    }
}
