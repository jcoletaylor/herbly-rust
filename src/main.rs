use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Pool;
use tide::http::cookies::SameSite;
use tide::prelude::*;
use tide::{Error, Server};
use uuid::Uuid;

mod controllers;
mod handlers;

use controllers::formulas as formulas_ctrl;
use controllers::herbs as herbs_ctrl;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
}
