#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[warn(
    clippy::nursery,
    clippy::style,
    clippy::unwrap_used,
    clippy::expect_used
)]
pub mod discord_rpc;
pub mod system_tray;
pub mod window;

use color_eyre::Result;
use discord_rpc::{Activity, DiscordRPCClient, User};
use system_tray::{create_system_tray, handle_system_tray_event};
use tokio::{sync::Mutex, time::Duration};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;
use window::handle_window_event;

#[tauri::command]
async fn try_connecting(state: tauri::State<'_, Mutex<DiscordRPCClient>>) -> Result<(), String> {
    state
        .lock()
        .await
        .try_connecting(Duration::from_secs(5), None)
        .await
        .map_err(|err| {
            error!("Error {err}");
            format!("{err}")
        })
}

#[tauri::command]
async fn handshake(
    state: tauri::State<'_, Mutex<DiscordRPCClient>>,
    client_id: String,
) -> Result<User, String> {
    state
        .lock()
        .await
        .handshake(client_id)
        .await
        .map_err(|err| {
            error!("Error {err}");
            format!("{err}")
        })
}

#[tauri::command]
async fn set_activity(
    state: tauri::State<'_, Mutex<DiscordRPCClient>>,
    activity: Activity,
) -> Result<String, String> {
    state
        .lock()
        .await
        .set_activity(Some(activity))
        .await
        .map_err(|err| {
            error!("Error {err}");
            format!("{err}")
        })
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )?;

    tauri::Builder::default()
        .setup(create_system_tray)
        .manage(Mutex::new(DiscordRPCClient::default()))
        .on_window_event(handle_window_event)
        .on_system_tray_event(handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            try_connecting,
            handshake,
            set_activity
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
