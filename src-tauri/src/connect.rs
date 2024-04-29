use std::net::SocketAddr;
use std::path::PathBuf;

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use http_body_util::{BodyExt, combinators::BoxBody, Full};
use hyper::{Method, Request, Response, StatusCode};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

static NOTFOUND: &[u8] = b"Not Found";

pub async fn send_file(addr: String, file_path: String) -> Result<String, String> {
	let mut conn = TcpStream::connect(addr).await.map_err(|e| { e.to_string() })?;
	let server_port = 3333;
	let server_addr = SocketAddr::from((conn.local_addr().unwrap().ip(), server_port));
	let listener = TcpListener::bind(server_addr).await.map_err(|e| { e.to_string() })?;
	let url = format!("http://{}:{}/{}", conn.local_addr().unwrap().ip(), server_port, file_path);
	println!("{}", url);
	let mut len_bytes = [0u8; 4];
	BigEndian::write_u32(&mut len_bytes, url.len() as u32);
	let mut buffer = len_bytes.to_vec();
	buffer.extend_from_slice(url.as_bytes());
	conn.write_all(&buffer).await.map_err(|e| { e.to_string() })?;

	let (stream, _) = listener.accept().await.map_err(|e| { e.to_string() })?;
	let io = TokioIo::new(stream);
	tokio::task::spawn(async move {
		if let Err(err) = http1::Builder::new()
			.serve_connection(io, service_fn(response_with_file))
			.await
		{
			println!("Failed to serve connection: {:?}", err);
		}
	});
	Ok("Installed".to_string())
}

async fn response_with_file(
	req: Request<hyper::body::Incoming>,
) -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> {
	match (req.method(), req.uri().path()) {
		(&Method::GET, _) => sendfile_http(req.uri().path()).await,
		_ => Ok(not_found()),
	}
}

/// HTTP status code 404
fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
	Response::builder()
		.status(StatusCode::NOT_FOUND)
		.body(Full::new(NOTFOUND.into()).map_err(|e| match e {}).boxed())
		.unwrap()
}

async fn sendfile_http(filename: &str) -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> {
	let filename = PathBuf::from(format!(".{}", filename));
	let file = File::open(filename).await;
	if file.is_err() {
		eprintln!("ERROR: Unable to open file.");
		return Ok(not_found());
	}
	let mut file: File = file.unwrap();
	let mut buf = Vec::new();
	file.read_to_end(&mut buf).await.unwrap();
	let bytes = bytes::Bytes::from(buf);
	let response = Response::builder()
		.status(StatusCode::OK)
		.body(Full::new(bytes).map_err(|e| match e {}).boxed())
		.unwrap();
	Ok(response)
}



