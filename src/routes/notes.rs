use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, post, delete, put};
use serde::Deserialize;
use serde_json::json;

use crate::db::NoteService;
use crate::types::Tags;
use crate::{db, AppState};

#[get("/notes/")]
pub async fn notes_home(state: web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().body("ok")
}
#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<Tags>,

}

#[post("/notes/create")]
pub async fn create_note(
    state: web::Data<AppState>, 
    note_data: web::Json<CreateNoteRequest>
) -> impl Responder {
    // Create the NoteService instance
    let note_service = NoteService::new(&state.db).await.unwrap();

    // Call the service to create the note
    let result = note_service
        .create_note(note_data.title.clone(), note_data.content.clone(), note_data.tags.clone())
        .await;

    match result {
        Ok(inserted_id) => HttpResponse::Created().json(json!({"id":inserted_id})), // Return the inserted ID as a response
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create note: {:?}", e)),
    }
}
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_note);
    cfg.service(notes_home);
}