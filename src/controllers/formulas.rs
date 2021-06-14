use tide::{Body, Response};

use crate::handlers;
use crate::helpers;
use crate::state;

pub async fn get_one(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let name = req.param("name")?;
    let formula = handlers::formulas::get_one(format!("{}", name), &db_pool).await?;
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&formula)?);
    Ok(res)
}

pub async fn get_all(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let bounds: helpers::LimitOffsetBounds = req.query()?;
    let limit: i64 = bounds.limit.unwrap_or(100);
    let offset: i64 = bounds.offset.unwrap_or(0);
    let formulas = handlers::formulas::get_all(limit, offset, &db_pool).await?;

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&formulas)?);
    Ok(res)
}
