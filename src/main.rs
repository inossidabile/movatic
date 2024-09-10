mod controllers;
mod models;
mod schema;
mod services;

use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};
use controllers::{stations_get, stations_index, stations_update};
use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub fn setup_db() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let database_url = env::var("DATABASE_URL").context("No DB url specified")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .context("Failed to create pool")?;

    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = setup_db().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(stations_index)
            .service(stations_get)
            .service(stations_update)
    })
    .bind(("0.0.0.0", 3003))?
    .run()
    .await
}
