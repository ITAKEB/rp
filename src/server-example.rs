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
    //let addr = SocketAddr::from(([127,0,0,1],3000));
    let addr = SocketAddr::from(([0,0,0,0],3000));
    /*
    let url = "https://httpbin.org/json".parse().context("Parsing URL")?;
    let res = client.get(url).await.context("Performing HTTP request")?;
    */
    
    let make_svc = make_service_fn(|_| async {
        Ok::<_,Infallible>(service_fn(|_req| async{
            Ok::<_,Infallible>(Response::new(Body::from("Hello world")))
        }))
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

*/