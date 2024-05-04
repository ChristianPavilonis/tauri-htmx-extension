use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};

#[tauri::command]
fn background_stuff(app: AppHandle) {
    thread::spawn(move || {
        for i in 0..=100 {
            app.emit("progress", format!("progress: {i}%")).unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    });
}

#[tauri::command]
fn meet(name: &str) -> String {
    format!("htmx, meet {}", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![meet, background_stuff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
