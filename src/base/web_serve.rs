mod api;
mod web_serve;

use actix_web::{HttpServer,App, web, Responder};

#[actix_rt::main]
async fn webserve () -> std::io::Result<()> {
    HttpServer::new()
}