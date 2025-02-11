#![allow(unused)]

use axum::response::Html;
use axum::Router;
use axum::routing::get;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

#[tokio::main]
async fn main(){
    let routes_hello = Router::new().route(
        "/hello",get(|| async {Html("Hello <strong>World!!!!</strong>")}),
    );

    let addr = SocketAddr::from(([127, 0 , 0, 1],8080));
    println!("->>Listening on {addr}\n");

    axum::Server::bind(&addr)
    .serve(routes_hello.into_make_service())
    .await
    .unwrap();


}