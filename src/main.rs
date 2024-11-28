use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct HomeResponse{
    message: String
}

#[get("/")]
async fn home() -> impl Responder{
    let response = HomeResponse{message:String::from("Hello world!")};
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}