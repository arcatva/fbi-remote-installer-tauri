use tokio::net::TcpStream;

pub async fn connect_tcp(addr: String) -> Result<String, String> {
    match TcpStream::connect(addr).await {
        Ok(_) => Ok("Connected to 3DS".to_string()),
        Err(_) => Err("Not connected".to_string()),
    }
}
