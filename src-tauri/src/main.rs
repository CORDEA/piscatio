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

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![client_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
