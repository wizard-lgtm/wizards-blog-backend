use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Database;
use routes::notes::configure;
use serde::Serialize;
use dotenv::dotenv;
use std::env;
mod db;
mod types;
mod routes;
mod utils;
use crate::routes::notes;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};


pub struct Auth;
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware{ service }))
    }
}


pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

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
            .wrap(Auth)
            .service(home)
            .configure(notes::configure)
            
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}