use actix_web::cookie::time::OffsetDateTime;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use serde::*;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Queryable, Selectable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = crate::schema::stations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Station {
    pub station_id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub is_renting: Option<bool>,
    pub is_returning: Option<bool>,
    pub is_installed: Option<bool>,
    pub num_docks_available: Option<i64>,
    pub num_bikes_available: Option<i64>,
    pub last_reported: Option<OffsetDateTime>,
}
