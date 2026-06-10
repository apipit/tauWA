#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BadgeCount {
    count: i32,
}

#[tauri::command]
fn set_badge_count(window: tauri::Window, count: i32) {
    // Set badge count on the window
    #[cfg(target_os = "macos")]
    {
        use tauri::Manager;
        let app_handle = window.app_handle();
        app_handle.set_badge_count(count).unwrap();
    }
    
    // For other platforms, you can use system tray or custom implementation
    println!("Badge count set to: {}", count);
}

#[tauri::command]
fn show_notification(window: tauri::Window, title: String, body: String) {
    use tauri::api::notification;
    notification::Notification::new(&window.config().tauri.bundle.identifier)
        .title(title)
        .body(body)
        .show()
        .unwrap();
}

fn create_system_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show WhatsApp"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(menu)
}

fn main() {
    tauri::Builder::default()
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![set_badge_count, show_notification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
