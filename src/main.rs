use tide::prelude::*;

mod app;
mod controllers;
mod handlers;
mod helpers;
mod models;
mod state;

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    tide::log::start();

    let db_pool = app::get_db_pool().await;
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
