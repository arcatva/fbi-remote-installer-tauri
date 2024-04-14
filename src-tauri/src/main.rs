// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use fbi_remote_installer_tauri::{list_files_with_path, FileMeta};

#[tauri::command(async)]
async fn list_files() -> Result<Vec<FileMeta>, ()> {
    Ok(list_files_with_path(".").await.unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
