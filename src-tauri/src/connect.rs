use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

pub async fn connect_tcp(addr: String) -> Result<String, String> {
	match TcpStream::connect(addr).await {
		Ok(_) => Ok("Connected to 3DS".to_string()),
		Err(_) => Err("Not connected".to_string()),
	}
}


async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
	Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}


pub async fn sendlink_tcp(addr: String, file_path: String) -> Result<String, String> {
	//let stream = TcpStream::connect(addr).await.map_err(|e| { e.to_string() })?;
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = TcpListener::bind(addr).await.map_err(|e| { e.to_string() })?;
	let (s, _) = listener.accept().await.map_err(|e| { e.to_string() })?;
	let io = TokioIo::new(s);
	tokio::task::spawn(async move {
		// Finally, we bind the incoming connection to our `hello` service
		if let Err(err) = http1::Builder::new()
			// `service_fn` converts our function in a `Service`
			.serve_connection(io, service_fn(hello))
			.await
		{
			eprintln!("Error serving connection: {:?}", err);
		}
	});
	Ok("Installed".to_string())
}










