/*
use std::convert::Infallible;
use hyper::{Client, Server, Response, Body, Request};
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
#[tokio::main]

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = hyper::Client::builder().build(https);
    let url = "http://localhost:3000".parse();

    let addr = SocketAddr::from(([0,0,0,0],3000));
    
    let server_handle = tokio::spawn(async move {
        let make_svc = make_service_fn(|_| async {
            Ok::<_,Infallible>(service_fn(|_req| async{
                Ok::<_,Infallible>(Response::new(Body::from("Hello world")))
            }))
        });
        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });
    println!("Spleeping");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("Finished");
    let res = client.get(url.unwrap()).await;
    println!("{:?}", res);


}

*/