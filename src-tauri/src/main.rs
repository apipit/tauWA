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

#[tauri::command]
fn update_unread_count(state: tauri::State<AppState>, count: i32) {
    let mut unread = state.unread_count.lock().unwrap();
    *unread = count;
    println!("Unread count updated: {}", count);
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
        .invoke_handler(tauri::generate_handler![get_unread_count, update_unread_count])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // Inject JavaScript to monitor title changes
            // WhatsApp Web updates title to "(3) WhatsApp" when there are unread messages
            let js_code = r#"
                (function() {
                    let lastTitle = document.title;
                    
                    function checkTitle() {
                        const currentTitle = document.title;
                        if (currentTitle !== lastTitle) {
                            lastTitle = currentTitle;
                            
                            // Parse unread count from title
                            let unreadCount = 0;
                            const match = currentTitle.match(/^\((\d+)\)/);
                            if (match) {
                                unreadCount = parseInt(match[1]);
                            }
                            
                            // Send to Tauri backend
                            if (window.__TAURI__) {
                                window.__TAURI__.invoke('update_unread_count', { count: unreadCount });
                            }
                            
                            console.log('Title changed:', currentTitle, 'Unread:', unreadCount);
                        }
                    }
                    
                    // Check every 2 seconds
                    setInterval(checkTitle, 2000);
                    
                    // Initial check
                    setTimeout(checkTitle, 3000);
                })();
            "#;
            
            // Execute JavaScript in the window
            window.eval(js_code).unwrap();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
