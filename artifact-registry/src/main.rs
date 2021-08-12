use hyper::{StatusCode, Method, Server, Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};

use tokio_util::codec::{BytesCodec, FramedRead};

use std::net::SocketAddr;
use std::convert::Infallible;


#[tokio::main]
pub async fn main()
{
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 7878)); // set to 127.0.0.1:7878

    let service = make_service_fn(|_| async
    {
        Ok::<_, Infallible>(service_fn(handle))        
    });

    let server = Server::bind(&address).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible>
{
    let method: &Method = request.method();

    if let Method::GET = *method
    {
        let uri_path: &str = request.uri().path();

        let file_path: String = format!("src/{}", uri_path);

        match tokio::fs::File::open(file_path).await
        {
            Ok(file) => {
                let codec: BytesCodec = BytesCodec::new();
                let stream: FramedRead<tokio::fs::File, BytesCodec> = FramedRead::new(file, codec);
                let body: Body = Body::wrap_stream(stream);

                Ok(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Access-Control-Allow-Origin", "http://localhost:8080")
                        .header("content-type", "application/wasm")
                        .body(body)
                        .expect("failed to set file as body for response.")
                )
            },
            Err(_) => Ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Access-Control-Allow-Origin", "http://localhost:8080")
                    .body(
                        Body::empty()
                    ).expect("failed to set empty body for response.")
            )
        }
    } else {
        Ok(
            Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .header("Access-Control-Allow-Origin", "http://localhost:8080")
                .body(
                    Body::empty()
                ).expect("failed to set empty body for response.")
        )
    }
}

