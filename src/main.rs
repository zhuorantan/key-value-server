use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
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

#[post("{path:.*}")]
async fn update(path: web::Path<String>, payload: String, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut storage = state.storage.lock().unwrap();
    storage.update(path.into_inner(), payload);
    Ok(HttpResponse::Ok().body(format!("{:?}", storage)))
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
            .service(update)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
