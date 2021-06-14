use tide::prelude::*;

mod controllers;
mod handlers;
mod helpers;
mod models;
mod state;
mod app;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    tide::log::start();
    
    let db_pool = app::make_db_pool().await;
    let server = app::make_server(db_pool).await;
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let mut listener = server
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("can't bind port");

    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await.unwrap();
}
