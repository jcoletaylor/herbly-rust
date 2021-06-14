use sqlx::PgPool;
use sqlx::Pool;
use tide::prelude::*;
use tide::Server;

mod controllers;
mod handlers;
mod helpers;
mod models;
mod state;

use controllers::formulas;
use controllers::herbs;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    tide::log::start();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let db_pool = make_db_pool(&db_url).await;

    let app = server(db_pool).await;

    let mut listener = app
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("can't bind port");

    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }

    listener.accept().await.unwrap();
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.unwrap()
}

async fn server(db_pool: PgPool) -> Server<state::State> {
    let state = state::State { db_pool };

    let mut app = tide::with_state(state);

    app.at("/herbs").get(herbs::get_all);
    app.at("/herbs/:name").get(herbs::get_one);
    app.at("/formulas").get(formulas::get_all);
    app.at("/formulas/:name").get(formulas::get_one);

    app
}
