use actix_web::{web, App, HttpResponse, HttpServer, Responder, delete, get, post, put};
use darkbird::document::{Document, FullText, Indexer, MaterializedView, Range, RangeField, Tags};
use darkbird::{Storage, StorageType, Options};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

type Pid = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    fullname: String,
}

impl User {
    pub fn new(fullname: &str) -> Self {
        User { fullname: fullname.to_owned() }
    }
}

// Required Document trait implementations
impl Document for User {}

impl Indexer for User {
    fn extract(&self) -> Vec<String> {
        vec![self.fullname.clone()]  // Index fullname for search
    }
}

impl Tags for User {
    fn get_tags(&self) -> Vec<String> {
        vec![]  // No tags needed for this example
    }
}

impl Range for User {
    fn get_fields(&self) -> Vec<RangeField> {
        vec![]  // No range fields needed
    }
}

impl MaterializedView for User {
    fn filter(&self) -> Option<String> {
        None  // No materialized view filtering
    }
}

impl FullText for User {
    fn get_content(&self) -> Option<String> {
        None  // No full-text content
    }
}

struct AppState {
    db: Arc<Storage<Pid, User>>,
}

#[post("/users")]
async fn create_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let pid = format!("user_{}", uuid::Uuid::new_v4());
    let user = user.into_inner();
    match data.db.insert(pid.clone(), user).await {
        Ok(_) => HttpResponse::Ok().json(pid),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

#[get("/users/{pid}")]
async fn get_user(data: web::Data<AppState>, pid: web::Path<String>) -> impl Responder {
    match data.db.lookup(&pid) {
        Some(user_ref) => HttpResponse::Ok().json(user_ref.value().clone()),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/users/{pid}")]
async fn update_user(data: web::Data<AppState>, pid: web::Path<String>, user: web::Json<User>) -> impl Responder {
    let user = user.into_inner();
    match data.db.insert(pid.clone(), user).await {
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}

#[delete("/users/{pid}")]
async fn delete_user(data: web::Data<AppState>, pid: web::Path<String>) -> impl Responder {
    let pid = pid.into_inner();
    match data.db.remove(pid).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting user"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = ".";
    let storage_name = "blackbird";
    let total_page_size = 1000;
    let stype = StorageType::RamCopies;
    let ops = Options::new(path, storage_name, total_page_size, stype, true);
    let db = Arc::new(Storage::<Pid, User>::open(ops).await.unwrap());

    let app_state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}