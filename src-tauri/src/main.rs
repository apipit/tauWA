#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent};
use std::sync::Mutex;

struct AppState {
    unread_count: Mutex<i32>,
}

#[tauri::command]
fn get_unread_count(state: tauri::State<AppState>) -> i32 {
    *state.unread_count.lock().unwrap()
}

fn create_system_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show WhatsApp"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(menu)
}

fn main() {
    let app_state = AppState {
        unread_count: Mutex::new(0),
    };

    tauri::Builder::default()
        .manage(app_state)
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // Minimize to system tray instead of closing
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_unread_count])
        .setup(|app| {
            // Set custom user agent to avoid detection issues
            let window = app.get_window("main").unwrap();
            
            // Listen for title changes to detect unread messages
            // WhatsApp Web updates title to "(3) WhatsApp" when there are unread messages
            let app_handle = app.handle();
            window.on_title_change(move |title| {
                let unread = parse_unread_from_title(title);
                println!("Title changed: {} -> Unread: {}", title, unread);
                
                // Update badge on macOS
                #[cfg(target_os = "macos")]
                {
                    app_handle.set_badge_count(unread).unwrap();
                }
                
                // Update system tray tooltip
                if unread > 0 {
                    println!("You have {} unread messages", unread);
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn parse_unread_from_title(title: &str) -> i32 {
    // WhatsApp Web format: "(3) WhatsApp" or "WhatsApp"
    if title.starts_with('(') {
        if let Some(end_paren) = title.find(')') {
            let count_str = &title[1..end_paren];
            if let Ok(count) = count_str.parse::<i32>() {
                return count;
            }
        }
    }
    0
}
