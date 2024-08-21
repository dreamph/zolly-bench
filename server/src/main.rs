use actix_web::{get, Result, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Listen :8000")
}

#[get("/v1/hello")]
async fn hello() -> Result<impl Responder> {
    let obj = MyObj {
        name: "Hello World!".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}