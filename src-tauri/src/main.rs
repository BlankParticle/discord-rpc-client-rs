#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[warn(clippy::unwrap_used, clippy::expect_used)]
pub mod discord_rpc;
pub mod system_tray;
pub mod window;

use crate::system_tray::{create_system_tray, handle_system_tray_event};
use color_eyre::Result;
use discord_rpc::{Activity, ActivityArgs, DiscordRPCClient};
use serde_json::from_str;
use std::process;
use tauri::Manager;
use tokio::{sync::mpsc, time::Duration};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;
use window::handle_window_event;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )?;
    tauri::Builder::default()
        .setup(|app| {
            create_system_tray(app)?;
            let (tx, mut rx) = mpsc::channel(1);

            app.listen_global("set-activity", move |data| {
                if let Some(payload) = data.payload() {
                    if let Ok(activity) = from_str::<Activity>(payload) {
                        tokio::task::block_in_place(|| {
                            tauri::async_runtime::block_on(async {
                                tx.send(activity).await.unwrap();
                            })
                        });
                    } else {
                        println!("Error in data");
                    }
                }
            });

            tokio::spawn(async move {
                let client =
                    DiscordRPCClient::new(1113164486161997925, Duration::from_secs(5), None).await;
                if let Some(mut client) = client {
                    match client.handshake().await {
                        Ok(payload) => println!("{:#?}", payload),
                        Err(err) => error!("Error: {}", err),
                    }
                    while let Some(activity) = rx.recv().await {
                        println!("Activity");
                        client
                            .set_activity(ActivityArgs {
                                pid: process::id(),
                                activity: Some(activity),
                            })
                            .await
                            .unwrap();
                    }
                    // loop {}
                }
            });
            Ok(())
        })
        .on_window_event(handle_window_event)
        .on_system_tray_event(handle_system_tray_event)
        .run(tauri::generate_context!())?;
    Ok(())
}
