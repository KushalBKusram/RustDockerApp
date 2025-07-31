use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
    id: String,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateNoteRequest {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct UpdateNoteRequest {
    title: Option<String>,
    content: Option<String>,
}

struct AppState {
    notes: Mutex<HashMap<String, Note>>,
}

async fn create_note(
    data: web::Data<AppState>,
    note_req: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse> {
    let mut notes = data.notes.lock().unwrap();
    let note = Note {
        id: Uuid::new_v4().to_string(),
        title: note_req.title.clone(),
        content: note_req.content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let note_id = note.id.clone();
    notes.insert(note.id.clone(), note.clone());
    
    Ok(HttpResponse::Created()
        .json(note))
}

async fn get_notes(data: web::Data<AppState>) -> Result<HttpResponse> {
    let notes = data.notes.lock().unwrap();
    let notes_vec: Vec<&Note> = notes.values().collect();
    Ok(HttpResponse::Ok().json(notes_vec))
}

async fn get_note(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let notes = data.notes.lock().unwrap();
    match notes.get(&id.into_inner()) {
        Some(note) => Ok(HttpResponse::Ok().json(note)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

async fn update_note(
    data: web::Data<AppState>,
    id: web::Path<String>,
    update_req: web::Json<UpdateNoteRequest>,
) -> Result<HttpResponse> {
    let mut notes = data.notes.lock().unwrap();
    if let Some(note) = notes.get_mut(&id.into_inner()) {
        if let Some(title) = &update_req.title {
            note.title = title.clone();
        }
        if let Some(content) = &update_req.content {
            note.content = content.clone();
        }
        note.updated_at = Utc::now();
        Ok(HttpResponse::Ok().json(note))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

async fn delete_note(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let mut notes = data.notes.lock().unwrap();
    if notes.remove(&id.into_inner()).is_some() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        notes: Mutex::new(HashMap::new()),
    });

    println!("Server running at http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(|| async {
                HttpResponse::Found()
                    .append_header(("Location", "/static/index.html"))
                    .finish()
            }))
            .route("/health", web::get().to(get_health_status))
            .route("/notes", web::post().to(create_note))
            .route("/notes", web::get().to(get_notes))
            .route("/notes/{id}", web::get().to(get_note))
            .route("/notes/{id}", web::put().to(update_note))
            .route("/notes/{id}", web::delete().to(delete_note))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}