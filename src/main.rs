use actix_web::{get, patch, web, App, Error, HttpResponse, HttpServer, delete};
use clap::Parser;
use std::sync::Mutex;

mod storage;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host to bind to
    #[clap(short, long, env = "KV_SERVER_HOST", default_value = "localhost")]
    host: String,

    /// Port to bind to
    #[clap(short, long, env = "KV_SERVER_PORT", default_value_t = 8124)]
    port: u16,

    /// Path to the database
    #[clap(short, long, env = "KV_SERVER_FILE_PATH")]
    file_path: Option<String>,
}

struct AppState {
    storage: Mutex<storage::Storage>,
}

#[get("{path:.*}")]
async fn get(path: web::Path<String>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let key_path = &path.into_inner();
    let storage = state.storage.lock().unwrap();
    let value = storage.get(key_path);

    match value {
        Some(value) => {
            println!("Returning value {} for path {}", value, key_path);
            Ok(HttpResponse::Ok().json(value))
        }
        None => {
            println!("No value found for path {}", key_path);
            Ok(HttpResponse::NotFound().finish())
        },
    }
}

async fn update(path: web::Path<String>, payload: String, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let key_path = &path.into_inner();
    let mut storage = state.storage.lock().unwrap();
    let result = storage.update(key_path, &payload);

    match result {
        Ok(()) => {
            println!("Updated value {} for path {}", payload, key_path);
            Ok(HttpResponse::Ok().finish())
        },
        Err(error) => Ok(handle_error(&error, &key_path)),
    }
}

#[patch("{path:.*}")]
async fn patch(path: web::Path<String>, payload: String, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let key_path = &path.into_inner();
    let mut storage = state.storage.lock().unwrap();
    let result = storage.append(key_path, &payload);

    match result {
        Ok(()) => {
            println!("Patched value {} for path {}", payload, key_path);
            Ok(HttpResponse::Ok().finish())
        },
        Err(error) => Ok(handle_error(&error, &key_path)),
    }
}

#[delete("{path:.*}")]
async fn delete(path: web::Path<String>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let key_path = &path.into_inner();
    let mut storage = state.storage.lock().unwrap();
    let result = storage.delete(key_path);

    match result {
        Ok(()) => {
            println!("Deleted value for path {}", key_path);
            Ok(HttpResponse::Ok().finish())
        },
        Err(error) => Ok(handle_error(&error, &key_path)),
    }
}

fn handle_error(error: &storage::Error, path: &str) -> HttpResponse {
    match error {
        storage::Error::NotAnObject(key) => {
            println!("Key {} is not an object", key);
            HttpResponse::BadRequest().body(format!("{} is not an object", key))
        }
        storage::Error::NoKey => {
            println!("No key found for path {}", path);
            HttpResponse::BadRequest().body("No key provided")
        }
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let storage = storage::Storage::from_file(args.file_path);

    println!("Loaded storage with data {}", storage.get("").unwrap().to_string());

    let state = web::Data::new(AppState {
        storage: Mutex::new(storage),
    });

    println!("Listening to {}:{}", args.host, args.port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .route("{path:.*}", web::post().to(update))
            .route("{path:.*}", web::put().to(update))
            .service(patch)
            .service(delete)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
