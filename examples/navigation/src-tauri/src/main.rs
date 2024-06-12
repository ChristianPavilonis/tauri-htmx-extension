// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]

use shtml::{html, Component, Elements, Render};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![home, page1, page2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn home() -> String {
    html! {
        <Layout>
            home
        </Layout>
    }.to_string()
}

#[tauri::command]
fn page1() -> String {
    html! {
        <Layout>
            page 1
        </Layout>
    }.to_string()
}

#[tauri::command]
fn page2() -> String {
    html! {
        <Layout>
            page 2
        </Layout>
    }.to_string()
}

fn Layout(elements: Elements) -> Component {
    html! {
        <div>
            <nav>
                <Link to="home">
                    home
                </Link>
                <Link to="page1">
                    page 1
                </Link>
                <Link to="page2">
                    page 2
                </Link>
            </nav>
            {elements}
        </div>
    }
}

fn Link(to: &str, elements: Elements) -> Component {
    html! {
        <a tauri-invoke=to hx-target="body">
            {elements}
        </a>
    }
}
