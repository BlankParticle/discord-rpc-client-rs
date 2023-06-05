// use crate::discord_rpc::{
//     client::{ActivityArgs, Client},
//     models::{Activity, ActivityTimestamps},
// };
use std::process;
use tauri::{App, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn create_system_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    SystemTray::new()
        .with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("toggle", "Hide Window"))
                // .add_item(CustomMenuItem::new("test", "Test"))
                .add_item(CustomMenuItem::new("quit", "Quit")),
        )
        .build(app)?;
    Ok(())
}

pub fn handle_system_tray_event(app: &AppHandle, ev: SystemTrayEvent) {
    match ev {
        SystemTrayEvent::LeftClick { .. } => {
            let window = app.get_window("main");
            if let Some(window) = window {
                window.show().unwrap_or_default();
                let item_handle = window.app_handle().tray_handle().get_item("toggle");
                item_handle.set_title("Hide Window").unwrap_or_default();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match &id[..] {
            "quit" => process::exit(0),
            // "test" => test(),
            "toggle" => {
                if let Some(window) = app.get_window("main") {
                    let item_handle = app.tray_handle().get_item(&id);
                    if window.is_visible().unwrap_or(false) {
                        window.hide().unwrap_or_default();
                        item_handle.set_title("Show Window").unwrap_or_default();
                    } else {
                        window.show().unwrap_or_default();
                        item_handle.set_title("Hide Window").unwrap_or_default();
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

// fn test() {
//     let mut client = Client::new(1113164486161997925);
//     client.start();
//     client
//         .set_activity(ActivityArgs {
//             pid: process::id(),
//             activity: Some(Activity {
//                 details: Some(String::from("Hello")),
//                 state: Some(String::from("I am Dying")),
//                 timestamps: Some(ActivityTimestamps {
//                     start: Some(
//                         SystemTime::now()
//                             .duration_since(SystemTime::UNIX_EPOCH)
//                             .unwrap()
//                             .as_millis(),
//                     ),
//                     end: None,
//                 }),
//                 ..Default::default()
//             }),
//         })
//         .unwrap();
// }
