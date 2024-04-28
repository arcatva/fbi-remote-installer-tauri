use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{BodyExt, combinators::BoxBody, Full, StreamBody};
use hyper::{Method, Request, Response, StatusCode};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::io::ReaderStream;

static NOTFOUND: &[u8] = b"Not Found";

pub async fn connect_tcp(addr: String) -> Result<String, String> {
	match TcpStream::connect(addr).await {
		Ok(_) => Ok("Connected to 3DS".to_string()),
		Err(_) => Err("Not connected".to_string()),
	}
}


pub async fn sendfile(addr: String, file_path: String) -> Result<String, String> {
	//let stream = TcpStream::connect(addr).await.map_err(|e| { e.to_string() })?;
	let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
	let listener = TcpListener::bind(addr).await.map_err(|e| { e.to_string() })?;
	let (stream, _) = listener.accept().await.map_err(|e| { e.to_string() })?;
	let io = TokioIo::new(stream);
	tokio::task::spawn(async move {
		if let Err(err) = http1::Builder::new()
			.serve_connection(io, service_fn(response_examples))
			.await
		{
			println!("Failed to serve connection: {:?}", err);
		}
	});


	Ok("Installed".to_string())
}

async fn response_examples(
	req: Request<hyper::body::Incoming>,
) -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> {
	match (req.method(), req.uri().path()) {
		(&Method::GET, _) => simple_file_send(req.uri().path()).await,
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

async fn simple_file_send(filename: &str) -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> {
	// Open file for reading
	let file = File::open(filename).await;
	if file.is_err() {
		eprintln!("ERROR: Unable to open file.");
		return Ok(not_found());
	}

	let mut file: File = file.unwrap();
	//let reader_stream = ReaderStream::new(file);

	// Convert to http_body_util::BoxBody
	//let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
	let mut buf = Vec::new();
	file.read_to_end(&mut buf).await.unwrap();
	let bytes = bytes::Bytes::from(buf);

	//let boxed_body = stream_body.boxed();

	// Send response
	let response = Response::builder()
		.status(StatusCode::OK)
		.body(Full::new(bytes).map_err(|e| match e {}).boxed())
		.unwrap();

	Ok(response)
}