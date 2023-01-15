#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;
use std::env;

#[tauri::command]
fn client_id() -> String {
    return env::var("INSTAGRAM_CLIENT_ID").expect("Failed to get the client ID.");
}

#[tauri::command]
fn client_secret() -> String {
    return env::var("INSTAGRAM_CLIENT_SECRET").expect("Failed to get the client secret.");
}

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![client_id, client_secret])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
