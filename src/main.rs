use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use mongodb::Database;
use serde::Serialize;
use dotenv::dotenv;
use std::env;
mod db;
mod types;


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
    dotenv().ok();

    let mongodb_uri: String = match env::var("MONGODB_URI") {
    Ok(uri) => uri,
    Err(_) => {
        eprintln!("Error: Environment variable MONGODB_URI is not set or invalid");
        std::process::exit(1);
    }
};
    let port: u16 = env::var("PORT").expect("PORT enviroment is missing").parse().unwrap();
    let db: Database = db::db_connect(mongodb_uri).await;

    println!("Connected to database: {}", db.name());
    println!("Server starting on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}