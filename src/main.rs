#![allow(unused)] // For beginning only

use std::{
    net::SocketAddr};

use axum::{
    routing::get,
    Router,
    response::Html    
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main(){
    let app = Router::new().route(
        "/hello",
        get(|| async {Html("Hello <strong>World!!!</strong>")}),
    );

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> Listening on {addr}\n");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 