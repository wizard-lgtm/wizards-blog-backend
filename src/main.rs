use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Database;
use routes::notes::configure;
use serde::Serialize;
use dotenv::dotenv;
use std::env;
mod db;
mod types;
mod routes;
use crate::routes::notes;

// Struct for the app state that will hold the database connection.
pub struct AppState {
    db: Database,
}

#[derive(Serialize)]
struct HomeResponse{
    message: String
}

#[get("/")]
async fn home(state: web::Data<AppState>) -> impl Responder{
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

    // Create the AppState with the database connection.
    let app_state = web::Data::new(AppState { db });

    println!("Server starting on port: {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(home)
            .configure(notes::configure)
            
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}