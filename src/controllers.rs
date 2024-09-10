use actix_web::{get, post, web, HttpResponse, Responder, Result};
use anyhow::Context;
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::{DbPool, Station};
use crate::services::stations_import;

#[derive(Deserialize)]
struct StationsUpdate {
    url: String,
}

#[derive(Deserialize)]
struct StationsGet {
    id: String,
}

#[get("/stations")]
async fn stations_index(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::stations::dsl::*;

    let connection = &mut pool.get().unwrap();
    let result: Vec<Station> = stations
        .select(Station::as_select())
        .load(connection)
        .context("Could not load stations")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(result))
}

#[get("/stations/{id}/status")]
async fn stations_get(
    pool: web::Data<DbPool>,
    station: web::Path<StationsGet>,
) -> Result<impl Responder> {
    use crate::schema::stations::dsl::*;

    let connection = &mut pool.get().unwrap();
    let result = stations
        .find(&station.id)
        .select(Station::as_select())
        .first(connection)
        .optional();

    match result {
        Ok(Some(station)) => Ok(web::Json(station)),
        Ok(None) => Err(actix_web::error::ErrorNotFound("Station not found")),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/stations")]
async fn stations_update(
    pool: web::Data<DbPool>,
    update: web::Json<StationsUpdate>,
) -> HttpResponse {
    actix_web::rt::spawn(stations_import(pool.clone(), update.url.clone()));
    HttpResponse::Ok().finish()
}
