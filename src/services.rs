use crate::models::DbPool;
use crate::schema::stations::dsl::stations;
use crate::schema::stations::*;
use actix_web::web;
use bigdecimal::BigDecimal;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;
use time::serde::timestamp;
use time::OffsetDateTime;

#[derive(Deserialize)]
struct Feed {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize)]
struct Feeds {
    feeds: Vec<Feed>,
}

#[derive(Deserialize)]
struct Information {
    pub station_id: String,
    pub address: String,
    pub name: String,
    pub lon: BigDecimal,
    pub lat: BigDecimal,
}

#[derive(Deserialize)]
struct Informations {
    stations: Vec<Information>,
}

#[derive(Deserialize)]
struct Status {
    pub station_id: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_renting: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_returning: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_installed: bool,
    num_docks_available: i64,
    num_bikes_available: i64,
    #[serde(with = "timestamp")]
    last_reported: OffsetDateTime,
}

#[derive(Deserialize)]
struct Statuses {
    stations: Vec<Status>,
}

#[derive(Deserialize)]
struct Wrapper<T> {
    data: T,
}

type FeedsIndex = Wrapper<HashMap<String, Feeds>>;
type InformationsIndex = Wrapper<Informations>;
type StatusesIndex = Wrapper<Statuses>;

pub async fn stations_import(pool: web::Data<DbPool>, url: String) -> Result<(), reqwest::Error> {
    println!("Importing stations from {}", url);
    let result = reqwest::get(url).await?.json::<FeedsIndex>().await?;

    for language in result.data.values() {
        for feed in &language.feeds {
            if feed.name == "station_information" {
                stations_import_information(pool.clone(), feed.url.clone()).await?;
            } else if feed.name == "station_status" {
                stations_import_status(pool.clone(), feed.url.clone()).await?;
            }
        }
    }

    Ok(())
}

pub async fn stations_import_information(
    pool: web::Data<DbPool>,
    url: String,
) -> Result<(), reqwest::Error> {
    let result = reqwest::get(url).await?.json::<InformationsIndex>().await?;

    let connection = &mut pool.get().unwrap();
    let entries = result
        .data
        .stations
        .into_iter()
        .map(|station| {
            (
                station_id.eq(station.station_id),
                address.eq(station.address),
                name.eq(station.name),
                longitude.eq(station.lon),
                latitude.eq(station.lat),
            )
        })
        .collect::<Vec<_>>();

    for entry in &entries {
        insert_into(stations)
            .values(entry)
            .on_conflict(station_id)
            .do_update()
            .set(entry.clone())
            .execute(connection)
            .expect("Error saving stations");
    }

    Ok(())
}

pub async fn stations_import_status(
    pool: web::Data<DbPool>,
    url: String,
) -> Result<(), reqwest::Error> {
    let result = reqwest::get(url).await?.json::<StatusesIndex>().await?;

    let connection = &mut pool.get().unwrap();
    let entries = result
        .data
        .stations
        .into_iter()
        .map(|status| {
            (
                station_id.eq(status.station_id),
                is_returning.eq(status.is_returning),
                is_renting.eq(status.is_renting),
                is_installed.eq(status.is_installed),
                num_docks_available.eq(status.num_docks_available),
                num_bikes_available.eq(status.num_bikes_available),
                last_reported.eq(status.last_reported),
            )
        })
        .collect::<Vec<_>>();

    for entry in &entries {
        insert_into(stations)
            .values(entry)
            .on_conflict(station_id)
            .do_update()
            .set(entry.clone())
            .execute(connection)
            .expect("Error saving stations");
    }

    Ok(())
}
