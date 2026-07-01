use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct LogBatch {
    service: String,
    host: String,
    entries: serde_json::Value,
}

#[post("/api/logs")]
async fn logs_receive(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is listening on localhost:8080");

    HttpServer::new(|| App::new().service(logs_receive))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
