use std::io;
use crate::http::server::start_http_server;
use actix_web::{Responder};


mod http;
mod global;
mod transaction;
mod decode;


#[actix::main]
async fn main() -> io::Result<()>{
    start_http_server().await
}


