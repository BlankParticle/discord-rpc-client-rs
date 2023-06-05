use color_eyre::{Report, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, json, Value};
use std::{env, io::Cursor, path::PathBuf};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ErrorKind, Interest},
    time::Duration,
};
use tracing::{error, info};
use uuid::Uuid;

#[cfg(unix)]
type Socket = tokio::net::UnixStream;

pub struct DiscordRPCClient {
    client_id: u64,
    socket: Socket,
    handshake_complete: bool,
}

impl DiscordRPCClient {
    pub async fn new(
        client_id: u64,
        reconnect_timeout: Duration,
        max_retry: Option<u32>,
    ) -> Option<DiscordRPCClient> {
        let mut reconnect_trials = 0u32;
        loop {
            let socket = Socket::connect(DiscordRPCClient::get_socket_path()).await;
            match socket {
                Ok(socket) => {
                    info!("Socket Connected");
                    break Some(DiscordRPCClient {
                        client_id,
                        socket,
                        handshake_complete: false,
                    });
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
        }
    }

    #[cfg(unix)]
    fn get_socket_path() -> PathBuf {
        let socket_dir = env::var("XDG_RUNTIME_DIR")
            .or_else(|_| env::var("TMPDIR"))
            .or_else(|_| match env::temp_dir().to_str() {
                Some(path) => Ok(path.to_string()),
                None => Err(()),
            })
            .unwrap_or(String::from("/tmp"));
        PathBuf::from(socket_dir).join("discord-ipc-0")
    }

    pub async fn handshake(&mut self) -> Result<User> {
        self.socket
            .ready(Interest::WRITABLE | Interest::READABLE)
            .await?;
        let (mut reader, mut writer) = self.socket.split();
        let send_payload = json!({
            "client_id":self.client_id.to_string(),
            "v": 1,
            "nonce":Uuid::new_v4().to_string()
        })
        .to_string();
        writer
            .write_all(&DiscordRPCClient::encode_message(OpCodes::Handshake, send_payload).await?)
            .await?;
        let mut recv_byte = [0; 2048];
        let bytes_read = reader.read(&mut recv_byte).await?;
        let recv_payload: &mut Value =
            &mut from_str(&DiscordRPCClient::decode_message(&recv_byte[..bytes_read]).await?)?;
        let user: User = from_value(recv_payload["data"]["user"].take())?;
        self.handshake_complete = true;
        Ok(user)
    }

    pub async fn set_activity(&mut self, activity: ActivityArgs) -> Result<()> {
        if !self.handshake_complete {
            return Err(Report::msg("Handshake not completed"));
        }
        let (mut reader, mut writer) = self.socket.split();
        let send_payload = json!({
            "cmd":"SET_ACTIVITY",
            "args": activity,
            "nonce": Uuid::new_v4().to_string()
        })
        .to_string();
        writer
            .write_all(&DiscordRPCClient::encode_message(OpCodes::Frame, send_payload).await?)
            .await?;
        let mut recv_byte = [0; 2048];
        let bytes_read = reader.read(&mut recv_byte).await?;
        let recv_payload = DiscordRPCClient::decode_message(&recv_byte[..bytes_read]).await?;
        println!("{recv_payload}");
        Ok(())
    }

    async fn encode_message(opcode: OpCodes, payload: String) -> Result<Vec<u8>> {
        let mut encoded_bytes = Vec::new();
        encoded_bytes.write_u32_le(opcode as u32).await?;
        encoded_bytes.write_u32_le(payload.len() as u32).await?;
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
pub struct ActivityArgs {
    pub pid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u128>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<(Option<u32>, Option<u32>)>,
}
