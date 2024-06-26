// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use components::{Layout, Main, TaskDescription};
use tauri::{AppHandle, Manager, Window};
use tokio::time::sleep;

mod components;

#[tauri::command]
async fn pomodoro(window: Window) -> Result<String, String> {
    tokio::spawn(async move {
        start_timer(window).await;
    });

    Ok("<span></span>".to_string())
}
#[tauri::command]
fn set_task_description(description: &str) -> String {
    TaskDescription(description).to_string()
}

#[tauri::command]
fn load_main_layout() -> String {
    Layout().to_string()
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            pomodoro,
            set_task_description,
            load_main_layout
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| match event {
        tauri::RunEvent::Ready => {}
        _ => {}
    });
}

async fn start_timer(window: Window) {
    let mut pomodoro_count = 0;
    // tick event
    loop {
        let mut seconds: u32 = 1500;
        window
            .emit(
                "tick",
                components::Timer(seconds, pomodoro_count).to_string(),
            )
            .ok();

        // 25 min work
        while seconds > 0 {
            window
                .emit(
                    "tick",
                    components::Timer(seconds, pomodoro_count).to_string(),
                )
                .ok();
            seconds -= 1;
            sleep(Duration::from_secs(1)).await;
        }

        pomodoro_count += 1;
        // 20 min break after 4 rounds
        if pomodoro_count == 4 {
            seconds = 1200;
            while seconds > 0 {
                window
                    .emit(
                        "tick",
                        components::Timer(seconds, pomodoro_count).to_string(),
                    )
                    .ok();
                seconds -= 1;
                sleep(Duration::from_secs(1)).await;
            }
        } else {
            // 5 min break
            seconds = 300;
            while seconds > 0 {
                window
                    .emit(
                        "tick",
                        components::Timer(seconds, pomodoro_count).to_string(),
                    )
                    .ok();
                seconds -= 1;
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
