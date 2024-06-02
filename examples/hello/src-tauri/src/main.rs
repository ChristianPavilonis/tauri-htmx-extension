use std::{
    fmt::format,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use tauri::{AppHandle, Manager, WebviewWindowBuilder};

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
    format!("<div id=\"meetme\" hx-swap-oob=\"true\">htmx, meet {}</div>", name)
}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![meet, background_stuff])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Ready => {
            app_handle.listen("stop", |event| {
                println!("{:?}", event);
                STOP.store(true, Ordering::Relaxed);
            });
            let app = app_handle.clone();
            thread::spawn(move || {
                let mut count = 0;
                while !STOP.load(Ordering::Relaxed) {
                    count += 1;
                    app.emit("tick", format!("tick: {count}")).unwrap();

                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
        _ => {}
    });
}
