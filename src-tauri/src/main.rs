// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


use hyper_util::rt::TokioExecutor;
use hyper_util::server::conn::auto;

mod connect;
mod file;

#[tauri::command(async)]
async fn list_files() -> Result<Vec<file::FileMeta>, String> {
	Ok(file::list_files(".").await?)
}


#[tauri::command(async)]
async fn send_file(addr: String, file_path: String) -> Result<String, String> {
	Ok(connect::send_file(addr, file_path).await?)
}

fn main() {
	auto::Builder::new(TokioExecutor::new());
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
            list_files,
            send_file,
        ])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
