use sqlx::{PgPool, Pool};
use tide::Server;

use crate::controllers::formulas;
use crate::controllers::herbs;
use crate::state;

pub async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = Pool::connect(&db_url).await.unwrap();
    db_pool
}

pub async fn make_server(db_pool: PgPool) -> Server<state::State> {
    let state = state::State { db_pool };
    let mut app = tide::with_state(state);
    app.at("/herbs").get(herbs::get_all);
    app.at("/herbs/:name").get(herbs::get_one);
    app.at("/formulas").get(formulas::get_all);
    app.at("/formulas/:name").get(formulas::get_one);

    app
}
