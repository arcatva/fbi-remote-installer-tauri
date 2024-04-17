// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod connect;
mod file;

#[tauri::command(async)]
async fn list_files() -> Result<Vec<file::FileMeta>, ()> {
    Ok(file::list_files(".").await.unwrap())
}

#[tauri::command(async)]
async fn connect_tcp(addr: String) -> Result<String, String> {
    Ok(connect::connect_tcp(addr).await?)
}

#[tauri::command(async)]
async fn sendfile_tcp(addr: String, file_path: String) -> Result<String, String> {
    Ok(connect::sendfile_tcp(addr, file_path).await?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_files,
            connect_tcp,
            sendfile_tcp
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
