use tauri::Window;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn connect_tcp(addr: String) -> Result<String, String> {
    match TcpStream::connect(addr).await {
        Ok(_) => Ok("Connected to 3DS".to_string()),
        Err(_) => Err("Not connected".to_string()),
    }
}

pub async fn sendfile_tcp(addr: String, file_path: String) -> Result<String, String> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| "Not connected".to_string())?;

    let mut file = File::open(file_path)
        .await
        .map_err(|e| "Failed to open the file".to_string())?;

    let total_size = file
        .metadata()
        .await
        .map_err(|e| "Failed to open the file".to_string())?
        .len();
    let mut total_read: u64 = 0;

    let mut buffer = vec![0; 4096];
    loop {
        let n = file
            .read_buf(&mut buffer)
            .await
            .map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        stream
            .write_all(&buffer[..n])
            .await
            .map_err(|e| e.to_string())?;
        total_read += n as u64;
        let progress = (total_read as f64 / total_size as f64) * 100.0;

        // Window::emit("progress", progress.to_string()).map_err(|e| e.to_string())?;
    }

    Ok("Installed".to_string())
}
