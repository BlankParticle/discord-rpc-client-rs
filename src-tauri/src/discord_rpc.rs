use color_eyre::{Report, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, json, Value};
use std::{env, io::Cursor, path::PathBuf, process};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ErrorKind, Interest},
    time::Duration,
};
use tracing::{error, info};
use uuid::Uuid;

#[cfg(unix)]
type Socket = tokio::net::UnixStream;

#[derive(Default, Debug)]
pub struct DiscordRPCClient {
    socket: Option<Socket>,
    pub handshake_done: bool,
}

impl DiscordRPCClient {
    pub async fn try_connecting(
        &mut self,
        reconnect_timeout: Duration,
        max_retry: Option<u32>,
    ) -> Result<()> {
        if self.socket.is_some() {
            return Ok(());
        }
        let mut reconnect_trials = 0u32;
        self.socket = loop {
            let socket = Socket::connect(Self::get_socket_path()).await;
            match socket {
                Ok(socket) => {
                    info!("Socket Connected");
                    break Some(socket);
                }
                Err(err) => match err.kind() {
                    ErrorKind::ConnectionRefused | ErrorKind::NotFound => {
                        reconnect_trials += 1;
                        if let Some(max_retry) = max_retry {
                            if reconnect_trials >= max_retry {
                                info!("Max trials for reconnect reached");
                                break None;
                            }
                        }
                        info!("Connection Error: {}", err);
                        info!("Trying to reconnect, trail {}", reconnect_trials);
                        tokio::time::sleep(reconnect_timeout).await;
                        continue;
                    }
                    e => {
                        error!("Unknown Error Occurred: {}", e);
                        break None;
                    }
                },
            }
        };
        Ok(())
    }

    #[cfg(unix)]
    fn get_socket_path() -> PathBuf {
        let socket_dir = env::var("XDG_RUNTIME_DIR")
            .or_else(|_| env::var("TMPDIR"))
            .or_else(|_| {
                env::temp_dir()
                    .to_str()
                    .map_or(Err(()), |path| Ok(path.to_string()))
            })
            .unwrap_or_else(|_| String::from("/tmp"));
        PathBuf::from(socket_dir).join("discord-ipc-0")
    }

    pub async fn handshake(&mut self, client_id: String) -> Result<User> {
        if let Some(ref mut socket) = self.socket {
            socket
                .ready(Interest::WRITABLE | Interest::READABLE)
                .await?;
            let (mut reader, mut writer) = socket.split();
            let send_payload = json!({
                "client_id":client_id,
                "v": 1,
                "nonce":Uuid::new_v4().to_string()
            })
            .to_string();
            writer
                .write_all(&Self::encode_message(OpCodes::Handshake, send_payload).await?)
                .await?;
            let mut recv_byte = [0; 2048];
            let bytes_read = reader.read(&mut recv_byte).await?;
            let recv_payload: &mut Value =
                &mut from_str(&Self::decode_message(&recv_byte[..bytes_read]).await?)?;
            let user: User = from_value(recv_payload["data"]["user"].take())?;
            self.handshake_done = true;
            Ok(user)
        } else {
            Err(Report::msg("Socket Not Connected"))
        }
    }

    pub async fn set_activity(&mut self, activity: Option<Activity>) -> Result<String> {
        if !self.handshake_done {
            return Err(Report::msg("Handshake not completed"));
        }
        if let Some(ref mut socket) = self.socket {
            let (mut reader, mut writer) = socket.split();
            let send_payload = json!({
                "cmd":"SET_ACTIVITY",
                "args": {
                    "pid": process::id(),
                    "activity": activity
                },
                "nonce": Uuid::new_v4().to_string()
            })
            .to_string();
            writer
                .write_all(&Self::encode_message(OpCodes::Frame, send_payload).await?)
                .await?;
            let mut recv_byte = [0; 2048];
            let bytes_read = reader.read(&mut recv_byte).await?;
            let recv_payload = Self::decode_message(&recv_byte[..bytes_read]).await?;
            println!("{recv_payload}");
            Ok(recv_payload)
        } else {
            Err(Report::msg("Socket not Connected"))
        }
    }

    async fn encode_message(opcode: OpCodes, payload: String) -> Result<Vec<u8>> {
        let mut encoded_bytes = Vec::new();
        encoded_bytes.write_u32_le(opcode as u32).await?;
        encoded_bytes
            .write_u32_le(u32::try_from(payload.len())?)
            .await?;
        encoded_bytes.write_all(payload.as_bytes()).await?;
        Ok(encoded_bytes)
    }

    async fn decode_message(bytes: &[u8]) -> Result<String> {
        let mut bytes = Cursor::new(bytes);
        let mut payload = String::new();
        bytes.read_u32_le().await?; //Opcode
        bytes.read_u32_le().await?; //Payload Length
        bytes.read_to_string(&mut payload).await?;
        Ok(payload)
    }
}

enum OpCodes {
    Handshake,
    Frame,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: String,
    flags: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub details: String,
    pub timestamps: ActivityTimestamps,
    pub assets: ActivityAssets,
    #[serde(skip_serializing_if = "Button::do_skip")]
    pub buttons: Vec<Button>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "ActivityTimestamps::do_skip")]
    pub start: u128,
    #[serde(skip_serializing_if = "ActivityTimestamps::do_skip")]
    pub end: u128,
}

impl ActivityTimestamps {
    const fn do_skip(timestamp: &u128) -> bool {
        *timestamp == 0
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub large_image: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub large_text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub small_image: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub small_text: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

impl Button {
    fn do_skip(vec: &Vec<Self>) -> bool {
        vec.is_empty() || vec.len() > 2
    }
}
