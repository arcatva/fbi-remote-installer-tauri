use std::io::Bytes;

use bytes::BytesMut;
use tauri::Window;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::join;
use tokio::net::TcpStream;
pub async fn connect_tcp(addr: String) -> Result<String, String> {
    match TcpStream::connect(addr).await {
        Ok(_) => Ok("Connected to 3DS".to_string()),
        Err(_) => Err("Not connected".to_string()),
    }
}

pub async fn sendfile_tcp(addr: String, file_path: String) -> Result<String, String> {
    let stream_future = TcpStream::connect(addr);

    let file_future = File::open(file_path);

    let (stream_result, file_result) = join!(stream_future, file_future);
    let (mut stream, mut file) = (
        stream_result.map_err(|e| e.to_string())?,
        file_result.map_err(|e| e.to_string())?,
    );
    let total_size = file
        .metadata()
        .await
        .map_err(|_| "Failed to open the file".to_string())?
        .len();
    let mut total_read: u64 = 0;

    let mut buffer = BytesMut::with_capacity(1024);
    loop {
        let n = file
            .read_buf(&mut buffer)
            .await
            .map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        stream
            .write_all(&buffer[..])
            .await
            .map_err(|e| e.to_string())?;
        total_read += n as u64;
        let progress = (total_read as f64 / total_size as f64) * 100.0;

        // Window::emit("progress", progress.to_string()).map_err(|e| e.to_string())?;
    }

    Ok("Installed".to_string())
}
