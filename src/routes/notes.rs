use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, post, delete, put};
use mongodb::bson::Uuid;
use serde::Deserialize;
use serde_json::json;

use crate::db::NoteService;
use crate::types::Tags;
use crate::{db, AppState};

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

#[get("/notes/")]
pub async fn find_note(
    state: web::Data<AppState>, 
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    // Extract page parameter from query string
    let page = match query.get("page") {
        Some(page_str) => match page_str.parse::<usize>() {
            Ok(page_num) if page_num > 0 => page_num,
            _ => return HttpResponse::BadRequest().body("Invalid or negative page number"),
        },
        None => 1, // Default to page 1 if not provided
    };

    // Create NoteService instance
    let note_service = NoteService::new(&state.db).await.unwrap();

    // Fetch notes by page
    match note_service.get_all_notes_by_page(page).await {
        Ok(notes) => HttpResponse::Ok().json(notes), // Return notes as JSON
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching notes: {:?}", e)),
    }
}

#[get("/note/{title}")]
pub async fn get_note(
    state: web::Data<AppState>, 
    title: web::Path<String>
) -> impl Responder {
    // Create the NoteService instance
    let note_service = NoteService::new(&state.db).await.unwrap();

    // Fetch the note by title
    let result = note_service.get_note_by_title(title.to_string()).await;

    match result {
        Ok(Some(note)) => HttpResponse::Ok().json(note), // Return the note as JSON if found
        Ok(None) => HttpResponse::NotFound().body("Note not found"), // Return 404 if note doesn't exist
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch note: {:?}", e)),
    }
}

#[delete("/notes/delete/{id}")]
pub async fn delete_note(
    state: web::Data<AppState>, 
    id: web::Path<String>
) -> impl Responder {
    let note_service = NoteService::new(&state.db).await.unwrap();
    
    let uuid_result = Uuid::parse_str(id.to_string());

    match uuid_result{
        Ok(uuid)=>{
            // try to delete the note
            let result = note_service.delete_note(uuid).await;
            
            match result {
                Ok(deleted_id) => HttpResponse::Ok().json(json!({"id": deleted_id})), // Return the deleted note's ID
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete note: {}", e.to_string())),
            }
        }
        Err(e)=>{
                HttpResponse::Forbidden().body(format!("Wrong uuid format {:?}", e))
        }
    }
 }



pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_note);
    cfg.service(get_note);
    cfg.service(find_note);
    cfg.service(delete_note);
}