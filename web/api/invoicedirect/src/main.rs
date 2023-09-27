mod db;
mod models;
mod handlers;
use handlers::init_db::init_db;
use actix_web::{App, HttpServer};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_db().expect("Failed to initialize the database");
    HttpServer::new(|| {
        App::new()
            // .configure(app::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}