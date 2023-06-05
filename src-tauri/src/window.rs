use tauri::{GlobalWindowEvent, Manager, WindowEvent};

pub fn handle_window_event(event: GlobalWindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event.event() {
        event.window().hide().unwrap_or_default();
        api.prevent_close();
        let item_handle = event.window().app_handle().tray_handle().get_item("toggle");
        item_handle.set_title("Show Window").unwrap_or_default();
    }
}
