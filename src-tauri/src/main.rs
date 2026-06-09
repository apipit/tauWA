#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // 1. Otomatis buka jendela DevTools Console saat startup
            window.open_devtools();

            // 2. Inject Custom CSS Styling
            let css_content = include_str!("../../src/inject.css");
            let _ = window.eval(&format!(
                "const style = document.createElement('style'); style.innerHTML = `{}`; document.head.appendChild(style);",
                css_content.replace('`', "\\`")
            ));

            // 3. Inject Custom JS Code Execution
            let js_content = include_str!("../../src/inject.js");
            let _ = window.eval(js_content);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Sembunyikan window ke background alih-alih mematikan proses database sync
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
