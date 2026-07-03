use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
struct LogBatch {
    service: String,
    host: String,
    entries: serde_json::Value,
}

#[post("/api/logs")]
async fn logs_receive(body: web::Bytes) -> impl Responder {
    let data: Value = match serde_json::from_slice(&body) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Parsing to JSON error: {:?}", e);
            return HttpResponse::BadRequest().body("Invalid JSON - quitting...");
        }
    };

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is listening on localhost:8080");

    HttpServer::new(|| App::new().service(logs_receive))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
