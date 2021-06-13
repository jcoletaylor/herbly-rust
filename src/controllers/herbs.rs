use crate::state;
use crate::models;

use tide::{Body, Request, Response};

use crate::handlers;

pub async fn get_one(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let name = req.param("name")?;
    // let row = handlers::models::get_one(name, &db_pool).await?;

    /*
    */

    let mut r = Response::new(200);
    r.set_body(format!("Herb name: {}", name));
    Ok(r)
}

pub async fn get_all(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let rows: Vec<models::Herb> = vec!();

    let mut r = Response::new(200);
    r.set_body(format!("All herbs"));
    Ok(r)
}