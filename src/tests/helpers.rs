#[cfg(test)]
use crate::app;
use crate::state;
use tide::Server;

pub async fn get_server() -> Server<state::State> {
    dotenv::dotenv().ok();
    let db_pool = app::get_db_pool().await;
    let server = app::make_server(db_pool).await;
    server
}
