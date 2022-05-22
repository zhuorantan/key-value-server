use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
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
}

struct AppState {
    storage: Mutex<storage::Storage>,
}

#[get("{path:.*}")]
async fn get(path: web::Path<String>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let storage = state.storage.lock().unwrap();
    let value = storage.get(path.into_inner());
    match value {
        Some(value) => Ok(HttpResponse::Ok().json(value)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

async fn update(path: web::Path<String>, payload: String, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut storage = state.storage.lock().unwrap();
    let result = storage.update(path.into_inner(), payload);
    match result {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(error) => match error {
            storage::Error::NotAnObject(key) => Ok(HttpResponse::BadRequest().body(format!("{} is not an object", key))),
        },
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Listening to {}:{}", args.host, args.port);

    let state = web::Data::new(AppState {
        storage: Mutex::new(storage::Storage::from_file().expect("Failed to load storage")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .route("{path:.*}", web::post().to(update))
            .route("{path:.*}", web::put().to(update))
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
