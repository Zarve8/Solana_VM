use std::io;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use crate::http::root::{
    send_transaction,
    new_vm,
    airdrop
};
use crate::http::rpc::rpc;


pub async fn start_http_server() -> io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(rpc)
            .service(send_transaction)
            .service(new_vm)
            .service(airdrop)
    })
        .bind(("127.0.0.1", 8080))? //127.0.0.1
        .workers(2)
        .run()
        .await
}